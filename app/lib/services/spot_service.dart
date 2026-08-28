import 'package:graphql_flutter/graphql_flutter.dart';
import '../core/constants.dart';
import 'package:latlong2/latlong.dart';
import 'auth_service.dart';

class Spot {
  final String id;
  final String? name;
  final double latitude;
  final double longitude;
  final int likesCount;
  final int commentsCount;
  final bool isLikedByMe;

  Spot({
    required this.id,
    this.name,
    required this.latitude,
    required this.longitude,
    this.likesCount = 0,
    this.commentsCount = 0,
    this.isLikedByMe = false,
  });

  factory Spot.fromJson(Map<String, dynamic> json) {
    return Spot(
      id: json['id'] as String,
      name: json['name'] as String?,
      latitude: (json['latitude'] as num).toDouble(),
      longitude: (json['longitude'] as num).toDouble(),
      likesCount: (json['likesCount'] as num?)?.toInt() ?? 0,
      commentsCount: (json['commentsCount'] as num?)?.toInt() ?? 0,
      isLikedByMe: json['isLikedByMe'] as bool? ?? false,
    );
  }

  LatLng get location => LatLng(latitude, longitude);
}

class SpotService {
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

  static Future<List<Spot>> fetchSpots() async {
    const String getSpotsQuery = r'''
      query {
        getSpots {
          id
          name
          latitude
          longitude
          likesCount
          commentsCount
          isLikedByMe
        }
      }
    ''';

    final client = await _getClient();
    final QueryOptions options = QueryOptions(
      document: gql(getSpotsQuery),
      fetchPolicy: FetchPolicy.networkOnly,
    );

    final QueryResult result = await client.query(options);

    if (result.hasException) {
      throw Exception(result.exception.toString());
    }

    final List<dynamic> spotsJson = result.data?['getSpots'] ?? [];
    return spotsJson.map((json) => Spot.fromJson(json)).toList();
  }

  static Future<Spot> fetchSpotById(String id) async {
    const String getSpotByIdQuery = r'''
      query GetSpotById($id: UUID!) {
        getSpotById(id: $id) {
          id
          name
          latitude
          longitude
          likesCount
          commentsCount
          isLikedByMe
        }
      }
    ''';

    final client = await _getClient();
    final QueryOptions options = QueryOptions(
      document: gql(getSpotByIdQuery),
      variables: {'id': id},
      fetchPolicy: FetchPolicy.networkOnly,
    );

    final QueryResult result = await client.query(options);

    if (result.hasException) {
      throw Exception(result.exception.toString());
    }

    if (result.data?['getSpotById'] == null) {
      throw Exception('Spot not found');
    }

    return Spot.fromJson(result.data!['getSpotById']);
  }
}
