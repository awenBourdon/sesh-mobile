import 'package:graphql_flutter/graphql_flutter.dart';
import '../core/constants.dart';

class Trick {
  final String id;
  final String userId;
  final String spotId;
  final String? description;
  final String? videoUrl;
  final DateTime createdAt;

  Trick({
    required this.id,
    required this.userId,
    required this.spotId,
    this.description,
    this.videoUrl,
    required this.createdAt,
  });

  factory Trick.fromJson(Map<String, dynamic> json) {
    return Trick(
      id: json['id'],
      userId: json['userId'],
      spotId: json['spotId'],
      description: json['description'],
      videoUrl: json['videoUrl'],
      createdAt: DateTime.parse(json['createdAt']),
    );
  }
}

class TrickService {
  static final HttpLink _httpLink = HttpLink(ApiConstants.graphqlUrl);

  static final GraphQLClient _client = GraphQLClient(
    link: _httpLink,
    cache: GraphQLCache(),
  );

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
        }
      }
    ''';

    final QueryOptions options = QueryOptions(
      document: gql(getAllTricksQuery),
      fetchPolicy: FetchPolicy.networkOnly,
    );

    final QueryResult result = await _client.query(options);

    if (result.hasException) {
      throw Exception(result.exception.toString());
    }

    final List<dynamic> tricksJson = result.data?['getAllTricks'] ?? [];
    return tricksJson.map((json) => Trick.fromJson(json)).toList();
  }
}
