import 'package:flutter/material.dart';
import '../services/spot_service.dart';
import '../services/trick_service.dart';
import '../services/social_service.dart';
import '../services/auth_service.dart';
import 'trick_detail_screen.dart';
import 'package:intl/intl.dart';

class SpotDetailScreen extends StatefulWidget {
  final String spotId;

  const SpotDetailScreen({super.key, required this.spotId});

  @override
  State<SpotDetailScreen> createState() => _SpotDetailScreenState();
}

class _SpotDetailScreenState extends State<SpotDetailScreen> {
  Spot? _spot;
  List<Comment> _comments = [];
  List<Trick> _tricks = [];
  bool _isLoading = true;
  bool _isLoadingComments = true;
  bool _isLoadingTricks = true;
  String? _currentUserId;
  String _errorMessage = '';
  final _commentController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _loadAllData();
  }

  @override
  void dispose() {
    _commentController.dispose();
    super.dispose();
  }

  Future<void> _loadAllData() async {
    _currentUserId = await AuthService.getUserId();
    await _loadSpotData();
    if (_spot != null) {
      _loadTricks();
      _loadComments();
    }
  }

  Future<void> _loadSpotData() async {
    try {
      final spot = await SpotService.fetchSpotById(widget.spotId);
      if (!mounted) return;
      setState(() {
        _spot = spot;
        _isLoading = false;
      });
    } catch (e) {
      if (!mounted) return;
      setState(() {
        _errorMessage = 'Erreur lors du chargement du spot : $e';
        _isLoading = false;
      });
    }
  }

  Future<void> _loadTricks() async {
    try {
      final tricks = await TrickService.fetchTricksBySpot(widget.spotId);
      if (!mounted) return;
      setState(() {
        _tricks = tricks;
        _isLoadingTricks = false;
      });
    } catch (e) {
      if (!mounted) return;
      setState(() => _isLoadingTricks = false);
    }
  }

  Future<void> _loadComments() async {
    try {
      final comments = await SocialService.fetchSpotComments(widget.spotId);
      if (!mounted) return;
      setState(() {
        _comments = comments;
        _isLoadingComments = false;
      });
    } catch (e) {
      if (!mounted) return;
      setState(() => _isLoadingComments = false);
    }
  }

  Future<void> _deleteComment(String commentId) async {
    try {
      final success = await SocialService.deleteSpotComment(commentId);
      if (success) {
        setState(() {
          _comments.removeWhere((c) => c.id == commentId);
        });
      } else {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Impossible de supprimer ce commentaire.')));
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Erreur : $e')));
      }
    }
  }

  Future<void> _handleLike() async {
    if (_spot == null) return;
    final oldIsLiked = _spot!.isLikedByMe;
    final oldLikesCount = _spot!.likesCount;
    setState(() {
      _spot = Spot(
        id: _spot!.id,
        name: _spot!.name,
        latitude: _spot!.latitude,
        longitude: _spot!.longitude,
        isLikedByMe: !oldIsLiked,
        likesCount: oldIsLiked ? oldLikesCount - 1 : oldLikesCount + 1,
        commentsCount: _spot!.commentsCount,
      );
    });
    try {
      await SocialService.toggleSpotLike(_spot!.id);
    } catch (e) {
      debugPrint('❌ SESH_LIKE_ERROR (Spot): $e');
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Erreur Like Spot: $e')),
        );
      }
      setState(() {
        _spot = Spot(
          id: _spot!.id,
          name: _spot!.name,
          latitude: _spot!.latitude,
          longitude: _spot!.longitude,
          isLikedByMe: oldIsLiked,
          likesCount: oldLikesCount,
          commentsCount: _spot!.commentsCount,
        );
      });
    }
  }

  Future<void> _addComment() async {
    final content = _commentController.text.trim();
    if (content.isEmpty) return;
    try {
      final newComment = await SocialService.addSpotComment(widget.spotId, content);
      setState(() {
        _comments.insert(0, newComment);
        _commentController.clear();
      });
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Erreur lors de l\'envoi')));
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) return const Scaffold(body: Center(child: CircularProgressIndicator(color: Color(0xFF1A1A1A))));
    if (_errorMessage.isNotEmpty || _spot == null) return Scaffold(appBar: AppBar(), body: Center(child: Text(_errorMessage.isEmpty ? 'SPOT INTROUVABLE' : _errorMessage)));

    return Scaffold(
      appBar: AppBar(
        title: Text(_spot!.name?.toUpperCase() ?? 'SPOT'),
        actions: [
          IconButton(
            icon: Icon(_spot!.isLikedByMe ? Icons.favorite : Icons.favorite_border),
            color: _spot!.isLikedByMe ? Colors.redAccent : const Color(0xFF1A1A1A),
            onPressed: _handleLike,
          ),
        ],
      ),
      body: Column(
        children: [
          Expanded(
            child: SingleChildScrollView(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Container(
                    height: 180,
                    width: double.infinity,
                    color: const Color(0xFF1A1A1A),
                    child: const Center(child: Icon(Icons.skateboarding, size: 60, color: Colors.white24)),
                  ),
                  Padding(
                    padding: const EdgeInsets.all(24),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(_spot!.name?.toUpperCase() ?? 'SPOT SANS NOM', style: const TextStyle(fontSize: 28, fontWeight: FontWeight.w900, letterSpacing: -1)),
                        const SizedBox(height: 8),
                        Text('${_spot!.likesCount} SKATEURS AIMENT CE SPOT', style: const TextStyle(fontSize: 10, fontWeight: FontWeight.bold, color: Colors.black45)),
                        
                        const SizedBox(height: 32),
                        const Text('TRICKS SUR CE SPOT', style: TextStyle(fontSize: 12, fontWeight: FontWeight.w900, color: Colors.black87)),
                        const SizedBox(height: 16),
                        _buildTricksSection(),

                        const SizedBox(height: 32),
                        const Text('COMMENTAIRES / ÉTAT DU SPOT', style: TextStyle(fontSize: 12, fontWeight: FontWeight.w900, color: Colors.black87)),
                        const SizedBox(height: 16),
                        _buildCommentsList(),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),
          _buildCommentInput(),
        ],
      ),
    );
  }

  Widget _buildTricksSection() {
    if (_isLoadingTricks) {
      return const Center(child: CircularProgressIndicator(color: Color(0xFF1A1A1A)));
    }
    if (_tricks.isEmpty) {
      return const Text('Aucun trick posté sur ce spot pour l\'instant.', style: TextStyle(color: Colors.grey, fontSize: 13));
    }

    return SizedBox(
      height: 160,
      child: ListView.builder(
        scrollDirection: Axis.horizontal,
        itemCount: _tricks.length,
        itemBuilder: (context, index) {
          final trick = _tricks[index];
          return GestureDetector(
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => TrickDetailScreen(trick: trick),
                ),
              );
            },
            child: Container(
              width: 140,
              margin: const EdgeInsets.only(right: 14),
              decoration: BoxDecoration(
                color: const Color(0xFF1A1A1A),
                borderRadius: BorderRadius.circular(16),
              ),
              clipBehavior: Clip.antiAlias,
              child: Stack(
                children: [
                  Positioned.fill(
                    child: trick.thumbnailUrl != null
                        ? Image.network(
                            trick.thumbnailUrl!,
                            fit: BoxFit.cover,
                            errorBuilder: (context, error, stackTrace) => const Center(
                              child: Icon(Icons.videocam_off, color: Colors.white24, size: 30),
                            ),
                          )
                        : const Center(
                            child: Icon(Icons.skateboarding, color: Colors.white24, size: 40),
                          ),
                  ),
                  Positioned.fill(
                    child: DecoratedBox(
                      decoration: BoxDecoration(
                        gradient: LinearGradient(
                          begin: Alignment.topCenter,
                          end: Alignment.bottomCenter,
                          colors: [
                            Colors.transparent,
                            Colors.black.withValues(alpha: 0.8),
                          ],
                          stops: const [0.4, 1.0],
                        ),
                      ),
                    ),
                  ),
                  Positioned(
                    bottom: 10,
                    left: 10,
                    right: 10,
                    child: Text(
                      trick.description?.toUpperCase() ?? 'TRICK',
                      maxLines: 2,
                      overflow: TextOverflow.ellipsis,
                      style: const TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.w900,
                        fontSize: 12,
                        letterSpacing: -0.3,
                      ),
                    ),
                  ),
                ],
              ),
            ),
          );
        },
      ),
    );
  }

  Widget _buildCommentsList() {
    if (_isLoadingComments) return const Center(child: CircularProgressIndicator(color: Color(0xFF1A1A1A)));
    if (_comments.isEmpty) return const Text('Aucun commentaire sur ce spot.', style: TextStyle(color: Colors.grey, fontSize: 13));

    return ListView.builder(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      itemCount: _comments.length,
      itemBuilder: (context, index) {
        final comment = _comments[index];
        final bool isMine = comment.userId == _currentUserId;
        return Padding(
          padding: const EdgeInsets.only(bottom: 16),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(DateFormat('dd.MM.yy HH:mm').format(comment.createdAt), style: const TextStyle(fontSize: 9, color: Colors.black26, fontWeight: FontWeight.bold)),
                    const SizedBox(height: 2),
                    Text(comment.content, style: const TextStyle(fontSize: 14)),
                  ],
                ),
              ),
              if (isMine)
                IconButton(
                  icon: const Icon(Icons.delete_outline, size: 18, color: Colors.black26),
                  onPressed: () => _deleteComment(comment.id),
                ),
            ],
          ),
        );
      },
    );
  }

  Widget _buildCommentInput() {
    return Container(
      padding: EdgeInsets.fromLTRB(16, 8, 16, MediaQuery.of(context).padding.bottom + 8),
      decoration: const BoxDecoration(color: Colors.white, border: Border(top: BorderSide(color: Colors.black12))),
      child: Row(
        children: [
          Expanded(
            child: TextField(
              controller: _commentController,
              decoration: InputDecoration(
                hintText: 'Une info sur le spot ?',
                filled: true,
                fillColor: const Color(0xFFF9F9F9),
                border: OutlineInputBorder(borderRadius: BorderRadius.circular(20), borderSide: BorderSide.none),
              ),
            ),
          ),
          const SizedBox(width: 8),
          IconButton(onPressed: _addComment, icon: const Icon(Icons.send, color: Color(0xFF1A1A1A))),
        ],
      ),
    );
  }
}
