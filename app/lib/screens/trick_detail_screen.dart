import 'package:flutter/material.dart';
import 'package:video_player/video_player.dart';
import 'package:chewie/chewie.dart';
import 'package:url_launcher/url_launcher.dart';
import '../services/trick_service.dart';
import '../services/social_service.dart';
import '../widgets/comments_sheet.dart';
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
  late int _likesCount;
  late bool _isLikedByMe;

  @override
  void initState() {
    super.initState();
    _likesCount = widget.trick.likesCount;
    _isLikedByMe = widget.trick.isLikedByMe;
    _initializePlayer();
  }

  Future<void> _handleLike() async {
    setState(() {
      if (_isLikedByMe) {
        _likesCount--;
        _isLikedByMe = false;
      } else {
        _likesCount++;
        _isLikedByMe = true;
      }
    });

    try {
      await SocialService.toggleLike(widget.trick.id);
    } catch (e) {
      setState(() {
        if (_isLikedByMe) {
          _likesCount--;
          _isLikedByMe = false;
        } else {
          _likesCount++;
          _isLikedByMe = true;
        }
      });
    }
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
            : Container(color: Colors.black),
        materialProgressColors: ChewieProgressColors(
          playedColor: Colors.white,
          handleColor: Colors.white,
          backgroundColor: Colors.white24,
          bufferedColor: Colors.white10,
        ),
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
      backgroundColor: const Color(0xFF0B0F17),
      appBar: AppBar(
        title: Text(widget.trick.description?.toUpperCase() ?? 'TRICK'),
        backgroundColor: Colors.transparent,
        foregroundColor: Colors.white,
      ),
      body: Column(
        children: [
          Expanded(
            flex: 3,
            child: Center(
              child: _isLoading
                  ? const CircularProgressIndicator(color: Colors.white)
                  : widget.trick.videoUrl != null && _chewieController != null
                      ? AspectRatio(
                          aspectRatio: _videoPlayerController!.value.aspectRatio,
                          child: Chewie(controller: _chewieController!),
                        )
                      : const Icon(Icons.videocam_off, size: 64, color: Colors.white24),
            ),
          ),
          Expanded(
            flex: 2,
            child: Container(
              width: double.infinity,
              padding: const EdgeInsets.all(24),
              decoration: const BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.vertical(top: Radius.circular(30)),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    widget.trick.description?.toUpperCase() ?? 'SANS DESCRIPTION',
                    style: const TextStyle(fontSize: 24, fontWeight: FontWeight.w900, letterSpacing: -1),
                  ),
                  const SizedBox(height: 12),
                  Row(
                    children: [
                      const Icon(Icons.calendar_today, size: 14, color: Colors.black38),
                      const SizedBox(width: 8),
                      Text(
                        'POSTÉ LE ${DateFormat('dd.MM.yyyy').format(widget.trick.createdAt)}',
                        style: const TextStyle(color: Colors.black38, fontSize: 12, fontWeight: FontWeight.bold),
                      ),
                    ],
                  ),
                  const SizedBox(height: 24),
                  Row(
                    children: [
                      _buildSocialAction(
                        _isLikedByMe ? Icons.favorite : Icons.favorite_border,
                        '$_likesCount',
                        _isLikedByMe ? Colors.redAccent : Colors.black87,
                        _handleLike,
                      ),
                      const SizedBox(width: 24),
                      _buildSocialAction(
                        Icons.chat_bubble_outline,
                        '${widget.trick.commentsCount}',
                        Colors.black87,
                        () {
                          showModalBottomSheet(
                            context: context,
                            isScrollControlled: true,
                            backgroundColor: Colors.transparent,
                            builder: (context) => CommentsSheet(trickId: widget.trick.id),
                          );
                        },
                      ),
                    ],
                  ),
                  const SizedBox(height: 20),
                  if (widget.trick.videoUrl != null)
                    TextButton(
                      onPressed: () async {
                        final url = Uri.parse(widget.trick.videoUrl!);
                        await launchUrl(url, mode: LaunchMode.externalApplication);
                      },
                      style: TextButton.styleFrom(padding: EdgeInsets.zero),
                      child: const Text(
                        "PROBLÈME DE LECTURE ? OUVRIR LA VIDÉO",
                        style: TextStyle(color: Colors.black45, fontSize: 10, decoration: TextDecoration.underline),
                      ),
                    ),
                  const Spacer(),
                  ElevatedButton(
                    onPressed: () => Navigator.pop(context),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: const Color(0xFF1A1A1A),
                      minimumSize: const Size(double.infinity, 60),
                    ),
                    child: const Text('RETOUR'),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSocialAction(IconData icon, String label, Color color, VoidCallback onTap) {
    return InkWell(
      onTap: onTap,
      child: Row(
        children: [
          Icon(icon, color: color, size: 24),
          const SizedBox(width: 8),
          Text(
            label,
            style: TextStyle(color: color, fontWeight: FontWeight.w900, fontSize: 16),
          ),
        ],
      ),
    );
  }
}
