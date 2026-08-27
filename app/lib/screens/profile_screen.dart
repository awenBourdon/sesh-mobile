import 'package:flutter/material.dart';
import '../services/auth_service.dart';

class ProfileScreen extends StatefulWidget {
  final VoidCallback onLogout;

  const ProfileScreen({super.key, required this.onLogout});

  @override
  State<ProfileScreen> createState() => _ProfileScreenState();
}

class _ProfileScreenState extends State<ProfileScreen> {
  String? _username;
  String? _email;
  String? _avatarUrl;
  bool _isAdmin = false;
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _loadUserData();
  }

  Future<void> _loadUserData() async {
    final username = await AuthService.getUsername();
    final email = await AuthService.getEmail();
    final isAdmin = await AuthService.isAdmin();
    final avatarUrl = await AuthService.getAvatarUrl();

    if (!mounted) return;

    setState(() {
      _username = username;
      _email = email;
      _isAdmin = isAdmin;
      _avatarUrl = avatarUrl;
      _isLoading = false;
    });
  }

  String _getInitials(String? name) {
    if (name == null || name.isEmpty) return '??';
    final parts = name.trim().split(' ');
    if (parts.length > 1) {
      return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
    }
    return name.substring(0, name.length >= 2 ? 2 : 1).toUpperCase();
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const Scaffold(
        body: Center(child: CircularProgressIndicator(color: Color(0xFF1A1A1A))),
      );
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('MON COMPTE'),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24),
        child: Column(
          children: [
            Container(
              padding: const EdgeInsets.all(4),
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                border: Border.all(color: const Color(0xFF1A1A1A), width: 2),
              ),
              child: CircleAvatar(
                radius: 60,
                backgroundColor: const Color(0xFFF0F0F0),
                backgroundImage: _avatarUrl != null ? NetworkImage(_avatarUrl!) : null,
                child: _avatarUrl == null
                    ? Text(
                        _getInitials(_username),
                        style: const TextStyle(fontSize: 40, color: Color(0xFF1A1A1A), fontWeight: FontWeight.w900),
                      )
                    : null,
              ),
            ),
            const SizedBox(height: 24),
            Text(
              _username?.toUpperCase() ?? 'UTILISATEUR',
              style: const TextStyle(fontSize: 28, fontWeight: FontWeight.w900, letterSpacing: -1),
            ),
            if (_isAdmin)
              Container(
                margin: const EdgeInsets.only(top: 12),
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
                decoration: BoxDecoration(
                  color: const Color(0xFF1A1A1A),
                  borderRadius: BorderRadius.circular(4),
                ),
                child: const Text(
                  'ADMIN',
                  style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 10),
                ),
              ),
            const SizedBox(height: 8),
            Text(
              _email ?? '',
              style: TextStyle(fontSize: 14, color: Colors.grey[600]),
            ),
            const SizedBox(height: 48),
            _buildMenuTile(Icons.history, 'MES SESHS', 'Historique des tricks'),
            _buildMenuTile(Icons.settings_outlined, 'PARAMÈTRES', 'Préférences et compte'),
            const SizedBox(height: 48),
            ElevatedButton(
              onPressed: () async {
                await AuthService.logout();
                widget.onLogout();
              },
              style: ElevatedButton.styleFrom(
                backgroundColor: Colors.white,
                foregroundColor: const Color(0xFF1A1A1A),
                side: const BorderSide(color: Colors.black12, width: 1),
                minimumSize: const Size(double.infinity, 60),
              ),
              child: const Text('DÉCONNEXION'),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildMenuTile(IconData icon, String title, String subtitle) {
    return Container(
      margin: const EdgeInsets.only(bottom: 16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(15),
        border: Border.all(color: Colors.black12),
      ),
      child: ListTile(
        leading: Icon(icon, color: const Color(0xFF1A1A1A)),
        title: Text(title, style: const TextStyle(fontWeight: FontWeight.w900, fontSize: 14)),
        subtitle: Text(subtitle, style: const TextStyle(fontSize: 12)),
        trailing: const Icon(Icons.chevron_right, color: Colors.black26),
        onTap: () {},
      ),
    );
  }
}
