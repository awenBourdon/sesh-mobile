import 'package:flutter/material.dart';
import 'package:latlong2/latlong.dart';
import '../services/trick_service.dart';
import '../services/upload_service.dart';
import 'location_picker_screen.dart';

class AddTrickScreen extends StatefulWidget {
  final LatLng initialLocation;

  const AddTrickScreen({super.key, required this.initialLocation});

  @override
  State<AddTrickScreen> createState() => _AddTrickScreenState();
}

class _AddTrickScreenState extends State<AddTrickScreen> {
  final _descriptionController = TextEditingController();
  late LatLng _selectedLocation;
  bool _isSubmitting = false;
  String? _videoUrl;

  @override
  void initState() {
    super.initState();
    _selectedLocation = widget.initialLocation;
  }

  @override
  void dispose() {
    _descriptionController.dispose();
    super.dispose();
  }

  Future<void> _pickVideo() async {
    setState(() => _isSubmitting = true);
    try {
      final result = await UploadService.pickAndUploadVideo(
        onProgress: (p) => debugPrint("Progress: $p"),
      );

      if (result != null) {
        setState(() {
          _videoUrl = result;
        });
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('VIDÉO CHARGÉE 🎬')),
          );
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('ERREUR VIDÉO : $e')),
        );
      }
    } finally {
      if (mounted) {
        setState(() {
          _isSubmitting = false;
        });
      }
    }
  }

  Future<void> _pickLocation() async {
    final LatLng? pickedLocation = await Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => LocationPickerScreen(initialCenter: _selectedLocation),
      ),
    );

    if (pickedLocation != null) {
      setState(() {
        _selectedLocation = pickedLocation;
      });
    }
  }

  Future<void> _submit() async {
    if (_descriptionController.text.trim().isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('DESCRIPTION REQUISE')),
      );
      return;
    }

    setState(() {
      _isSubmitting = true;
    });

    try {
      await TrickService.createTrick(
        latitude: _selectedLocation.latitude,
        longitude: _selectedLocation.longitude,
        description: _descriptionController.text.trim(),
        videoUrl: _videoUrl,
      );

      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('SESH ENVOYÉE ! 🛹')),
        );
        Navigator.pop(context, true);
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('ERREUR : $e')),
        );
      }
    } finally {
      if (mounted) {
        setState(() {
          _isSubmitting = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('NOUVELLE SESH'),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Text('OÙ ÇA ?', style: TextStyle(fontWeight: FontWeight.w900, fontSize: 12, color: Colors.black45)),
            const SizedBox(height: 12),
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(15),
                border: Border.all(color: Colors.black12),
              ),
              child: Row(
                children: [
                  const Icon(Icons.location_on, color: Color(0xFF1A1A1A)),
                  const SizedBox(width: 12),
                  Expanded(
                    child: Text(
                      '${_selectedLocation.latitude.toStringAsFixed(4)}, ${_selectedLocation.longitude.toStringAsFixed(4)}',
                      style: const TextStyle(fontFamily: 'monospace', fontWeight: FontWeight.bold),
                    ),
                  ),
                  IconButton(
                    onPressed: _isSubmitting ? null : _pickLocation,
                    icon: const Icon(Icons.edit_location_alt_outlined),
                    color: const Color(0xFF1A1A1A),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 32),
            const Text('CLIP VIDÉO', style: TextStyle(fontWeight: FontWeight.w900, fontSize: 12, color: Colors.black45)),
            const SizedBox(height: 12),
            if (_videoUrl == null)
              ElevatedButton.icon(
                onPressed: _isSubmitting ? null : _pickVideo,
                icon: const Icon(Icons.videocam_outlined),
                label: const Text('CHOISIR UNE VIDÉO'),
                style: ElevatedButton.styleFrom(
                  backgroundColor: const Color(0xFFF0F0F0),
                  foregroundColor: const Color(0xFF1A1A1A),
                  minimumSize: const Size(double.infinity, 80),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(15), side: const BorderSide(color: Colors.black12)),
                ),
              )
            else
              Container(
                padding: const EdgeInsets.all(16),
                decoration: BoxDecoration(
                  color: const Color(0xFFE6FFFA),
                  borderRadius: BorderRadius.circular(15),
                  border: Border.all(color: const Color(0xFF38B2AC)),
                ),
                child: const Row(
                  children: [
                    Icon(Icons.check_circle_outline, color: Color(0xFF38B2AC)),
                    SizedBox(width: 12),
                    Text('VIDÉO PRÊTE', style: TextStyle(color: Color(0xFF2C7A7B), fontWeight: FontWeight.w900)),
                  ],
                ),
              ),
            const SizedBox(height: 32),
            const Text('DESCRIPTION', style: TextStyle(fontWeight: FontWeight.w900, fontSize: 12, color: Colors.black45)),
            const SizedBox(height: 12),
            TextField(
              controller: _descriptionController,
              decoration: const InputDecoration(hintText: 'Ex: Heelflip over the rail'),
              maxLines: 3,
              enabled: !_isSubmitting,
            ),
            const SizedBox(height: 48),
            ElevatedButton(
              onPressed: _isSubmitting ? null : _submit,
              style: ElevatedButton.styleFrom(
                minimumSize: const Size(double.infinity, 65),
              ),
              child: _isSubmitting
                  ? const SizedBox(height: 25, width: 25, child: CircularProgressIndicator(color: Colors.white, strokeWidth: 3))
                  : const Text('PUBLIER LE TRICK'),
            ),
          ],
        ),
      ),
    );
  }
}
