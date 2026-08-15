import 'package:graphql_flutter/graphql_flutter.dart';
import '../core/constants.dart';
import 'package:latlong2/latlong.dart';

class Spot {
  final String id;
  final String? name;
  final double latitude;
  final double longitude;

  Spot({
    required this.id,
    this.name,
    required this.latitude,
    required this.longitude,
  });

  factory Spot.fromJson(Map<String, dynamic> json) {
    return Spot(
      id: json['id'],
      name: json['name'],
      latitude: json['latitude'].toDouble(),
      longitude: json['longitude'].toDouble(),
    );
  }

  LatLng get location => LatLng(latitude, longitude);
}

class SpotService {
  static final HttpLink _httpLink = HttpLink(ApiConstants.graphqlUrl);

  static final GraphQLClient _client = GraphQLClient(
    link: _httpLink,
    cache: GraphQLCache(),
  );

  static Future<List<Spot>> fetchSpots() async {
    const String getSpotsQuery = r'''
      query {
        getSpots {
          id
          name
          latitude
          longitude
        }
      }
    ''';

    final QueryOptions options = QueryOptions(
      document: gql(getSpotsQuery),
      fetchPolicy: FetchPolicy.networkOnly,
    );

    final QueryResult result = await _client.query(options);

    if (result.hasException) {
      throw Exception(result.exception.toString());
    }

    final List<dynamic> spotsJson = result.data?['getSpots'] ?? [];
    return spotsJson.map((json) => Spot.fromJson(json)).toList();
  }
}
