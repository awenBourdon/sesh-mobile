import 'package:flutter/material.dart';
import '../services/social_service.dart';
import '../services/auth_service.dart';
import 'package:intl/intl.dart';

class CommentsSheet extends StatefulWidget {
  final String trickId;

  const CommentsSheet({super.key, required this.trickId});

  @override
  State<CommentsSheet> createState() => _CommentsSheetState();
}

class _CommentsSheetState extends State<CommentsSheet> {
  final _commentController = TextEditingController();
  List<Comment> _comments = [];
  bool _isLoading = true;
  bool _isSending = false;
  String? _currentUserId;

  @override
  void initState() {
    super.initState();
    _loadData();
  }

  @override
  void dispose() {
    _commentController.dispose();
    super.dispose();
  }

  Future<void> _loadData() async {
    final userId = await AuthService.getUserId();
    try {
      final comments = await SocialService.fetchTrickComments(widget.trickId);
      if (!mounted) return;
      setState(() {
        _currentUserId = userId;
        _comments = comments;
        _isLoading = false;
      });
    } catch (e) {
      if (mounted) setState(() => _isLoading = false);
    }
  }

  Future<void> _sendComment() async {
    final content = _commentController.text.trim();
    if (content.isEmpty) return;
    setState(() => _isSending = true);
    try {
      final newComment = await SocialService.addComment(widget.trickId, content);
      setState(() {
        _comments.insert(0, newComment);
        _commentController.clear();
        _isSending = false;
      });
    } catch (e) {
      setState(() => _isSending = false);
    }
  }

  Future<void> _deleteComment(String commentId) async {
    try {
      final success = await SocialService.deleteComment(commentId);
      if (success) {
        setState(() {
          _comments.removeWhere((c) => c.id == commentId);
        });
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Erreur lors de la suppression')));
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: EdgeInsets.only(bottom: MediaQuery.of(context).viewInsets.bottom),
      height: MediaQuery.of(context).size.height * 0.7,
      decoration: const BoxDecoration(color: Colors.white, borderRadius: BorderRadius.vertical(top: Radius.circular(25))),
      child: Column(
        children: [
          Container(width: 40, height: 5, margin: const EdgeInsets.symmetric(vertical: 12), decoration: BoxDecoration(color: Colors.black12, borderRadius: BorderRadius.circular(5))),
          const Text('COMMENTAIRES', style: TextStyle(fontWeight: FontWeight.w900, fontSize: 14)),
          const Divider(),
          Expanded(
            child: _isLoading
                ? const Center(child: CircularProgressIndicator(color: Color(0xFF1A1A1A)))
                : _comments.isEmpty
                    ? const Center(child: Text('Aucun commentaire.', style: TextStyle(color: Colors.grey)))
                    : ListView.builder(
                        padding: const EdgeInsets.all(16),
                        itemCount: _comments.length,
                        itemBuilder: (context, index) {
                          final comment = _comments[index];
                          final bool isMine = comment.userId == _currentUserId;
                          return Padding(
                            padding: const EdgeInsets.only(bottom: 20),
                            child: Row(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                CircleAvatar(backgroundColor: const Color(0xFFF0F0F0), radius: 18, child: const Icon(Icons.person, size: 20, color: Colors.black26)),
                                const SizedBox(width: 12),
                                Expanded(
                                  child: Column(
                                    crossAxisAlignment: CrossAxisAlignment.start,
                                    children: [
                                      Text(DateFormat('dd.MM.yy HH:mm').format(comment.createdAt), style: const TextStyle(fontSize: 10, color: Colors.black38, fontWeight: FontWeight.bold)),
                                      const SizedBox(height: 2),
                                      Text(comment.content, style: const TextStyle(fontSize: 14, color: Color(0xFF1A1A1A))),
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
                      ),
          ),
          Padding(
            padding: const EdgeInsets.all(16),
            child: Row(
              children: [
                Expanded(child: TextField(controller: _commentController, decoration: InputDecoration(hintText: 'Ajouter un commentaire...', filled: true, fillColor: const Color(0xFFF9F9F9), border: OutlineInputBorder(borderRadius: BorderRadius.circular(25), borderSide: BorderSide.none)))),
                const SizedBox(width: 10),
                IconButton(onPressed: _isSending ? null : _sendComment, icon: _isSending ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2, color: Color(0xFF1A1A1A))) : const Icon(Icons.send, color: Color(0xFF1A1A1A))),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
