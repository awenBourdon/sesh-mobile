import 'package:flutter/material.dart';
import 'package:video_player/video_player.dart';
import 'package:chewie/chewie.dart';
import 'package:url_launcher/url_launcher.dart';
import '../services/trick_service.dart';
import 'package:intl/intl.dart';

class TrickDetailScreen extends StatefulWidget {
  final Trick trick;

  const TrickDetailScreen({super.key, required this.trick});

  @override
  State<TrickDetailScreen> createState() => _TrickDetailScreenState();
}

class _TrickDetailScreenState extends State<TrickDetailScreen> {
  VideoPlayerController? _videoPlayerController;
  ChewieController? _chewieController;
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _initializePlayer();
  }

  Future<void> _initializePlayer() async {
    if (widget.trick.videoUrl == null) {
      setState(() => _isLoading = false);
      return;
    }

    try {
      _videoPlayerController = VideoPlayerController.networkUrl(
        Uri.parse(widget.trick.videoUrl!),
      );

      await _videoPlayerController!.initialize();

      _chewieController = ChewieController(
        videoPlayerController: _videoPlayerController!,
        autoPlay: true,
        looping: true,
        aspectRatio: _videoPlayerController!.value.aspectRatio,
        placeholder: widget.trick.thumbnailUrl != null
            ? Image.network(widget.trick.thumbnailUrl!, fit: BoxFit.cover)
            : const Center(child: CircularProgressIndicator()),
      );

      setState(() => _isLoading = false);
    } catch (e) {
      setState(() => _isLoading = false);
    }
  }

  @override
  void dispose() {
    _videoPlayerController?.dispose();
    _chewieController?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      appBar: AppBar(
        title: Text(widget.trick.description ?? 'Trick'),
        backgroundColor: Colors.transparent,
        foregroundColor: Colors.white,
      ),
      body: Column(
        children: [
          // Zone Vidéo
          Expanded(
            flex: 3,
            child: Center(
              child: _isLoading
                  ? const CircularProgressIndicator(color: Colors.blueAccent)
                  : widget.trick.videoUrl != null && _chewieController != null
                      ? Chewie(controller: _chewieController!)
                      : const Column(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            Icon(Icons.videocam_off, size: 64, color: Colors.grey),
                            SizedBox(height: 10),
                            Text('Pas de vidéo disponible', style: TextStyle(color: Colors.grey)),
                          ],
                        ),
            ),
          ),
          // Zone Infos
          Expanded(
            flex: 2,
            child: Container(
              width: double.infinity,
              padding: const EdgeInsets.all(20),
              decoration: const BoxDecoration(
                color: Color(0xFF1A1F2E),
                borderRadius: BorderRadius.vertical(top: Radius.circular(30)),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    widget.trick.description ?? 'Pas de description',
                    style: const TextStyle(fontSize: 22, fontWeight: FontWeight.bold, color: Colors.white),
                  ),
                  const SizedBox(height: 10),
                  Row(
                    children: [
                      const Icon(Icons.calendar_today, size: 16, color: Colors.grey),
                      const SizedBox(width: 8),
                      Text(
                        'Posté le ${DateFormat('dd/MM/yyyy à HH:mm').format(widget.trick.createdAt)}',
                        style: const TextStyle(color: Colors.grey),
                      ),
                    ],
                  ),
                  const SizedBox(height: 15),
                  if (widget.trick.videoUrl != null)
                    Center(
                      child: TextButton.icon(
                        onPressed: () async {
                          final url = Uri.parse(widget.trick.videoUrl!);
                          await launchUrl(url, mode: LaunchMode.externalApplication);
                        },
                        icon: const Icon(Icons.open_in_new, color: Colors.blueAccent),
                        label: const Text(
                          "La vidéo ne charge pas ? Ouvrir en externe",
                          style: TextStyle(color: Colors.blueAccent, fontSize: 13),
                        ),
                      ),
                    ),
                  const Spacer(),
                  ElevatedButton.icon(
                    onPressed: () => Navigator.pop(context),
                    icon: const Icon(Icons.location_on),
                    label: const Text('VOIR LE SPOT'),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.blueAccent,
                      foregroundColor: Colors.white,
                      minimumSize: const Size(double.infinity, 50),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
