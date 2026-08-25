import 'package:flutter_dotenv/flutter_dotenv.dart';

class ApiConstants {
  static String get baseUrl => dotenv.get('API_BASE_URL', fallback: 'http://localhost:8080/api');
  static String get graphqlUrl => dotenv.get('GRAPHQL_URL', fallback: 'http://localhost:8080/graphql');
}
