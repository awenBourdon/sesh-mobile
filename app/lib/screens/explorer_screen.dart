import 'package:flutter/material.dart';
import '../services/trick_service.dart';
import 'package:intl/intl.dart';
import 'spot_detail_screen.dart';
import 'trick_detail_screen.dart';

class ExplorerScreen extends StatefulWidget {
  const ExplorerScreen({super.key});

  @override
  State<ExplorerScreen> createState() => _ExplorerScreenState();
}

class _ExplorerScreenState extends State<ExplorerScreen> {
  List<Trick> _tricks = [];
  bool _isLoading = true;
  String? _errorMessage;

  @override
  void initState() {
    super.initState();
    _loadTricks();
  }

  Future<void> _loadTricks() async {
    try {
      final tricks = await TrickService.fetchAllTricks();
      if (!mounted) return;
      setState(() {
        _tricks = tricks;
        _isLoading = false;
        _errorMessage = null;
      });
    } catch (e) {
      if (!mounted) return;
      setState(() {
        _isLoading = false;
        _errorMessage = 'Erreur lors du chargement des tricks';
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('EXPLORER'),
        centerTitle: true,
      ),
      body: _isLoading
          ? const Center(child: CircularProgressIndicator(color: Color(0xFF1A1A1A)))
          : RefreshIndicator(
              onRefresh: _loadTricks,
              color: const Color(0xFF1A1A1A),
              child: _errorMessage != null
                  ? ListView(
                      children: [
                        SizedBox(height: MediaQuery.of(context).size.height * 0.3),
                        Center(child: Text(_errorMessage!)),
                        TextButton(onPressed: _loadTricks, child: const Text('RÉESSAYER'))
                      ],
                    )
                  : _tricks.isEmpty
                      ? ListView(
                          physics: const AlwaysScrollableScrollPhysics(),
                          children: const [
                            SizedBox(height: 100),
                            Center(
                              child: Text(
                                'AUCUN TRICK VALIDÉ POUR LE MOMENT.\nTIREZ POUR ACTUALISER.',
                                textAlign: TextAlign.center,
                                style: TextStyle(color: Colors.grey, fontWeight: FontWeight.bold, fontSize: 12),
                              ),
                            ),
                          ],
                        )
                      : ListView.builder(
                          physics: const AlwaysScrollableScrollPhysics(),
                          padding: const EdgeInsets.all(16),
                          itemCount: _tricks.length,
                          itemBuilder: (context, index) {
                            final trick = _tricks[index];
                            return InkWell(
                              onTap: () {
                                Navigator.push(
                                  context,
                                  MaterialPageRoute(
                                    builder: (context) => TrickDetailScreen(trick: trick),
                                  ),
                                );
                              },
                              child: Container(
                                margin: const EdgeInsets.only(bottom: 20),
                                decoration: BoxDecoration(
                                  borderRadius: BorderRadius.circular(24),
                                  color: const Color(0xFF1A1A1A), // Fond noir pour les cartes
                                ),
                                clipBehavior: Clip.antiAlias,
                                child: Stack(
                                  children: [
                                    // Image ou Placeholder
                                    AspectRatio(
                                      aspectRatio: 1, // Format carré style Dice
                                      child: trick.thumbnailUrl != null
                                          ? Image.network(
                                              trick.thumbnailUrl!,
                                              fit: BoxFit.cover,
                                              errorBuilder: (context, error, stackTrace) => const Center(
                                                child: Icon(Icons.videocam_off, color: Colors.white24, size: 40),
                                              ),
                                            )
                                          : const Center(
                                              child: Icon(Icons.skateboarding, color: Colors.white24, size: 60),
                                            ),
                                    ),
                                    // Dégradé pour la lisibilité
                                    Positioned.fill(
                                      child: DecoratedBox(
                                        decoration: BoxDecoration(
                                          gradient: LinearGradient(
                                            begin: Alignment.topCenter,
                                            end: Alignment.bottomCenter,
                                            colors: [
                                              Colors.transparent,
                                              Colors.black.withValues(alpha: 0.7),
                                            ],
                                            stops: const [0.6, 1.0],
                                          ),
                                        ),
                                      ),
                                    ),
                                    // Informations
                                    Positioned(
                                      bottom: 0,
                                      left: 0,
                                      right: 0,
                                      child: Padding(
                                        padding: const EdgeInsets.all(20),
                                        child: Column(
                                          crossAxisAlignment: CrossAxisAlignment.start,
                                          children: [
                                            Text(
                                              trick.description?.toUpperCase() ?? 'TRICK SANS NOM',
                                              style: const TextStyle(
                                                color: Colors.white,
                                                fontWeight: FontWeight.w900,
                                                fontSize: 22,
                                                letterSpacing: -0.5,
                                              ),
                                            ),
                                            const SizedBox(height: 4),
                                            Text(
                                              DateFormat('dd.MM.yyyy').format(trick.createdAt),
                                              style: TextStyle(
                                                color: Colors.white.withValues(alpha: 0.6),
                                                fontSize: 12,
                                                fontWeight: FontWeight.bold,
                                              ),
                                            ),
                                          ],
                                        ),
                                      ),
                                    ),
                                  ],
                                ),
                              ),
                            );
                          },
                        ),
            ),
    );
  }
}
