import 'package:flutter/material.dart';
import '../services/spot_service.dart';

class SpotDetailScreen extends StatefulWidget {
  final String spotId;

  const SpotDetailScreen({super.key, required this.spotId});

  @override
  State<SpotDetailScreen> createState() => _SpotDetailScreenState();
}

class _SpotDetailScreenState extends State<SpotDetailScreen> {
  Spot? _spot;
  bool _isLoading = true;
  String _errorMessage = '';

  @override
  void initState() {
    super.initState();
    _loadSpotData();
  }

  Future<void> _loadSpotData() async {
    try {
      final spot = await SpotService.fetchSpotById(widget.spotId);
      setState(() {
        _spot = spot;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _errorMessage = 'Erreur lors du chargement du spot : $e';
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(_spot?.name?.toUpperCase() ?? 'SPOT'),
      ),
      body: _isLoading
          ? const Center(child: CircularProgressIndicator(color: Color(0xFF1A1A1A)))
          : _errorMessage.isNotEmpty
              ? Center(child: Text(_errorMessage, textAlign: TextAlign.center))
              : _spot == null
                  ? const Center(child: Text('SPOT INTROUVABLE'))
                  : Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Container(
                          height: 200,
                          width: double.infinity,
                          color: const Color(0xFF1A1A1A),
                          child: const Center(
                            child: Icon(Icons.skateboarding, size: 80, color: Colors.white24),
                          ),
                        ),
                        Padding(
                          padding: const EdgeInsets.all(24.0),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                _spot!.name?.toUpperCase() ?? 'SPOT SANS NOM',
                                style: const TextStyle(fontSize: 32, fontWeight: FontWeight.w900, letterSpacing: -1),
                              ),
                              const SizedBox(height: 8),
                              Container(
                                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
                                decoration: BoxDecoration(
                                  color: const Color(0xFFF0F0F0),
                                  borderRadius: BorderRadius.circular(4),
                                ),
                                child: Text(
                                  '${_spot!.latitude.toStringAsFixed(6)}, ${_spot!.longitude.toStringAsFixed(6)}',
                                  style: const TextStyle(fontFamily: 'monospace', fontSize: 12, color: Colors.black54),
                                ),
                              ),
                              const SizedBox(height: 40),
                              const Text(
                                'TRICKS SUR CE SPOT',
                                style: TextStyle(fontSize: 14, fontWeight: FontWeight.w900, color: Colors.black45),
                              ),
                              const SizedBox(height: 20),
                              const Center(
                                child: Text(
                                  'Aucun trick pour le moment.\nSoyez le premier !',
                                  textAlign: TextAlign.center,
                                  style: TextStyle(fontStyle: FontStyle.italic, color: Colors.grey),
                                ),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
    );
  }
}
