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
        title: Text(_spot?.name ?? 'Chargement...'),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Navigator.pop(context),
        ),
      ),
      body: _isLoading
          ? const Center(child: CircularProgressIndicator())
          : _errorMessage.isNotEmpty
              ? Center(child: Text(_errorMessage, textAlign: TextAlign.center))
              : _spot == null
                  ? const Center(child: Text('Spot introuvable'))
                  : Center(
                      child: Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                          const Icon(
                            Icons.skateboarding,
                            size: 100,
                            color: Colors.blueAccent,
                          ),
                          const SizedBox(height: 20),
                          Text(
                            _spot!.name ?? 'Spot sans nom',
                            style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
                          ),
                          const SizedBox(height: 10),
                          Text(
                            'Coordonnées : ${_spot!.latitude.toStringAsFixed(4)}, ${_spot!.longitude.toStringAsFixed(4)}',
                            style: TextStyle(color: Colors.grey[600], fontSize: 16),
                          ),
                          const SizedBox(height: 40),
                          const Text(
                            'Bientôt ici : La liste des tricks réalisés sur ce spot !',
                            textAlign: TextAlign.center,
                            style: TextStyle(fontStyle: FontStyle.italic),
                          ),
                        ],
                      ),
                    ),
    );
  }
}
