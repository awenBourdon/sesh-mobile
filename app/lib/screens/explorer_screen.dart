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

  @override
  void initState() {
    super.initState();
    _loadTricks();
  }

  Future<void> _loadTricks() async {
    try {
      final tricks = await TrickService.fetchAllTricks();
      setState(() {
        _tricks = tricks;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Explorer les Tricks'),
        centerTitle: true,
      ),
      body: _isLoading
          ? const Center(child: CircularProgressIndicator())
          : RefreshIndicator(
              onRefresh: _loadTricks,
              child: _tricks.isEmpty
                  ? ListView(
                      physics: const AlwaysScrollableScrollPhysics(),
                      children: const [
                        SizedBox(height: 100),
                        Center(
                          child: Text(
                            'Aucun trick validé pour le moment.\nTirez pour actualiser.',
                            textAlign: TextAlign.center,
                            style: TextStyle(color: Colors.grey),
                          ),
                        ),
                      ],
                    )
                  : ListView.builder(
                      physics: const AlwaysScrollableScrollPhysics(),
                      padding: const EdgeInsets.all(10),
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
                          child: Card(
                            elevation: 3,
                            margin: const EdgeInsets.only(bottom: 15),
                            shape: RoundedRectangleBorder(
                              borderRadius: BorderRadius.circular(12),
                            ),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                ListTile(
                                  leading: const CircleAvatar(
                                    backgroundColor: Colors.blueAccent,
                                    child: Icon(Icons.skateboarding, color: Colors.white),
                                  ),
                                  title: Text(
                                    trick.description ?? 'Trick sans description',
                                    style: const TextStyle(fontWeight: FontWeight.bold),
                                  ),
                                  subtitle: Text(
                                    'Le ${DateFormat('dd/MM/yyyy à HH:mm').format(trick.createdAt)}',
                                    style: const TextStyle(fontSize: 12),
                                  ),
                                ),
                                if (trick.videoUrl != null)
                                  const Padding(
                                    padding: EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                                    child: Row(
                                      children: [
                                        Icon(Icons.videocam, size: 16, color: Colors.grey),
                                        SizedBox(width: 5),
                                        Text('Vidéo disponible', style: TextStyle(color: Colors.grey)),
                                      ],
                                    ),
                                  ),
                                Padding(
                                  padding: const EdgeInsets.all(16),
                                  child: Row(
                                    mainAxisAlignment: MainAxisAlignment.end,
                                    children: [
                                      TextButton.icon(
                                        onPressed: () {
                                          Navigator.push(
                                            context,
                                            MaterialPageRoute(
                                              builder: (context) => SpotDetailScreen(spotId: trick.spotId),
                                            ),
                                          );
                                        },
                                        icon: const Icon(Icons.location_on, size: 18),
                                        label: const Text('Voir le spot'),
                                      ),
                                    ],
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
