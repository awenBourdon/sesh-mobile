import 'dart:convert';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:http/http.dart' as http;
import '../core/constants.dart';

class AuthService {
  static const _storage = FlutterSecureStorage();
  static const _tokenKey = 'jwt_token';
  static const _isAdminKey = 'is_admin';
  static const _userIdKey = 'user_id';
  static const _usernameKey = 'username';
  static const _emailKey = 'email';
  static const _avatarKey = 'avatar_url';

  static Future<String?> getToken() async {
    return await _storage.read(key: _tokenKey);
  }

  static Future<String?> getUsername() async {
    return await _storage.read(key: _usernameKey);
  }

  static Future<String?> getUserId() async {
    return await _storage.read(key: _userIdKey);
  }

  static Future<String?> getEmail() async {
    return await _storage.read(key: _emailKey);
  }

  static Future<String?> getAvatarUrl() async {
    return await _storage.read(key: _avatarKey);
  }

  static Future<bool> isAdmin() async {
    final isAdmin = await _storage.read(key: _isAdminKey);
    return isAdmin == 'true';
  }

  static Future<bool> login(String email, String password) async {
    final url = Uri.parse('${ApiConstants.baseUrl}/auth/login');
    try {
      final response = await http.post(
        url,
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'email': email.trim(),
          'password': password,
        }),
      );

      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        await _storage.write(key: _tokenKey, value: data['token']);
        await _storage.write(key: _userIdKey, value: data['user_id']);
        await _storage.write(key: _isAdminKey, value: data['is_admin'].toString());
        await _storage.write(key: _usernameKey, value: data['username']);
        await _storage.write(key: _emailKey, value: data['email']);
        if (data['avatar_url'] != null) {
          await _storage.write(key: _avatarKey, value: data['avatar_url']);
        }
        return true;
      }
      return false;
    } catch (e) {
      return false;
    }
  }

  static Future<bool> register(String email, String username, String password) async {
    final url = Uri.parse('${ApiConstants.baseUrl}/auth/register');
    try {
      final response = await http.post(
        url,
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'email': email.trim(),
          'username': username.trim(),
          'password': password,
        }),
      );

      if (response.statusCode == 201) {
        final data = jsonDecode(response.body);
        await _storage.write(key: _tokenKey, value: data['token']);
        await _storage.write(key: _userIdKey, value: data['user_id']);
        await _storage.write(key: _isAdminKey, value: data['is_admin'].toString());
        await _storage.write(key: _usernameKey, value: data['username']);
        await _storage.write(key: _emailKey, value: data['email']);
        if (data['avatar_url'] != null) {
          await _storage.write(key: _avatarKey, value: data['avatar_url']);
        }
        return true;
      }
      return false;
    } catch (e) {
      return false;
    }
  }

  static Future<void> logout() async {
    await _storage.delete(key: _tokenKey);
    await _storage.delete(key: _userIdKey);
    await _storage.delete(key: _isAdminKey);
    await _storage.delete(key: _usernameKey);
    await _storage.delete(key: _emailKey);
    await _storage.delete(key: _avatarKey);
  }
}