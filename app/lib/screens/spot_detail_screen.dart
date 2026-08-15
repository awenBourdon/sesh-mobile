import 'package:flutter/material.dart';
import '../services/spot_service.dart';

class SpotDetailScreen extends StatelessWidget {
  final Spot spot;

  const SpotDetailScreen({super.key, required this.spot});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(spot.name ?? 'Spot sans nom'),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Navigator.pop(context),
        ),
      ),
      body: Center(
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
              spot.name ?? 'Spot sans nom',
              style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 10),
            Text(
              'Coordonnées : ${spot.latitude.toStringAsFixed(4)}, ${spot.longitude.toStringAsFixed(4)}',
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
