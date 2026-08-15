import 'dart:io';
import 'package:flutter/foundation.dart';

class ApiConstants {
  static String get baseUrl {
    // TODO : à adapter en prod
    if (kIsWeb) {
      return 'http://localhost:8080/api';
    } else if (Platform.isAndroid) {
      return 'http://10.0.2.2:8080/api';
    } else {
      return 'http://localhost:8080/api';
    }
  }

  static String get graphqlUrl {
    if (kIsWeb) {
      return 'http://localhost:8080/graphql';
    } else if (Platform.isAndroid) {
      return 'http://10.0.2.2:8080/graphql';
    } else {
      return 'http://localhost:8080/graphql';
    }
  }
}
