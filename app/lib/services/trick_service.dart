import 'package:graphql_flutter/graphql_flutter.dart';
import '../core/constants.dart';
import 'auth_service.dart';

class Trick {
  final String id;
  final String userId;
  final String spotId;
  final String? description;
  final String? videoUrl;
  final DateTime createdAt;
  final int likesCount;
  final int commentsCount;
  final bool isLikedByMe;

  Trick({
    required this.id,
    required this.userId,
    required this.spotId,
    this.description,
    this.videoUrl,
    required this.createdAt,
    this.likesCount = 0,
    this.commentsCount = 0,
    this.isLikedByMe = false,
  });

  String? get thumbnailUrl {
    if (videoUrl == null) return null;
    return videoUrl!
        .replaceAll('.mp4', '.jpg')
        .replaceAll('/video/upload/', '/video/upload/so_auto,w_500,c_limit/');
  }

  factory Trick.fromJson(Map<String, dynamic> json) {
    return Trick(
      id: json['id'],
      userId: json['userId'],
      spotId: json['spotId'],
      description: json['description'],
      videoUrl: json['videoUrl'],
      createdAt: DateTime.parse(json['createdAt']),
      likesCount: json['likesCount'] ?? 0,
      commentsCount: json['commentsCount'] ?? 0,
      isLikedByMe: json['isLikedByMe'] ?? false,
    );
  }
}

class TrickService {
  static Future<GraphQLClient> _getClient() async {
    final token = await AuthService.getToken();
    final HttpLink httpLink = HttpLink(
      ApiConstants.graphqlUrl,
      defaultHeaders: {
        if (token != null) 'Authorization': 'Bearer $token',
      },
    );

    return GraphQLClient(
      link: httpLink,
      cache: GraphQLCache(),
    );
  }

  static Future<List<Trick>> fetchAllTricks() async {
    const String getAllTricksQuery = r'''
      query {
        getAllTricks {
          id
          userId
          spotId
          description
          videoUrl
          createdAt
          likesCount
          commentsCount
          isLikedByMe
        }
      }
    ''';

    final client = await _getClient();
    final QueryOptions options = QueryOptions(
      document: gql(getAllTricksQuery),
      fetchPolicy: FetchPolicy.networkOnly,
    );

    final QueryResult result = await client.query(options);

    if (result.hasException) {
      throw Exception(result.exception.toString());
    }

    final List<dynamic> tricksJson = result.data?['getAllTricks'] ?? [];
    return tricksJson.map((json) => Trick.fromJson(json)).toList();
  }

  static Future<Trick> createTrick({
    required double latitude,
    required double longitude,
    String? description,
    String? videoUrl,
  }) async {
    const String createTrickMutation = r'''
      mutation CreateTrick($input: CreateTrickInput!) {
        createTrick(input: $input) {
          id
          userId
          spotId
          description
          videoUrl
          createdAt
        }
      }
    ''';

    final client = await _getClient();
    final MutationOptions options = MutationOptions(
      document: gql(createTrickMutation),
      variables: {
        'input': {
          'latitude': latitude,
          'longitude': longitude,
          'description': description,
          'videoUrl': videoUrl,
        },
      },
    );

    final QueryResult result = await client.mutate(options);

    if (result.hasException) {
      throw Exception(result.exception.toString());
    }

    final data = result.data?['createTrick'];
    if (data == null) {
      throw Exception('Failed to create trick');
    }

    return Trick.fromJson(data);
  }
}
