import 'package:graphql_flutter/graphql_flutter.dart';
import '../core/constants.dart';
import 'auth_service.dart';

class Comment {
  final String id;
  final String userId;
  final String trickId;
  final String content;
  final DateTime createdAt;

  Comment({
    required this.id,
    required this.userId,
    required this.trickId,
    required this.content,
    required this.createdAt,
  });

  factory Comment.fromJson(Map<String, dynamic> json) {
    return Comment(
      id: json['id'],
      userId: json['userId'],
      trickId: json['trickId'],
      content: json['content'],
      createdAt: DateTime.parse(json['createdAt']),
    );
  }
}

class SocialService {
  static Future<GraphQLClient> _getClient() async {
    final token = await AuthService.getToken();
    final HttpLink httpLink = HttpLink(
      ApiConstants.graphqlUrl,
      defaultHeaders: {
        if (token != null) 'Authorization': 'Bearer $token',
      },
    );

    return GraphQLClient(
      link: httpLink,
      cache: GraphQLCache(),
    );
  }

  static Future<bool> toggleLike(String trickId) async {
    const String toggleLikeMutation = r'''
      mutation ToggleLike($trickId: UUID!) {
        toggleLike(trickId: $trickId)
      }
    ''';

    final client = await _getClient();
    final MutationOptions options = MutationOptions(
      document: gql(toggleLikeMutation),
      variables: {'trickId': trickId},
    );

    final QueryResult result = await client.mutate(options);

    if (result.hasException) {
      throw Exception(result.exception.toString());
    }

    return result.data?['toggleLike'] ?? false;
  }

  static Future<bool> toggleSpotLike(String spotId) async {
    const String toggleSpotLikeMutation = r'''
      mutation ToggleSpotLike($spotId: UUID!) {
        toggleSpotLike(spotId: $spotId)
      }
    ''';

    final client = await _getClient();
    final MutationOptions options = MutationOptions(
      document: gql(toggleSpotLikeMutation),
      variables: {'spotId': spotId},
    );

    final QueryResult result = await client.mutate(options);

    if (result.hasException) {
      throw Exception(result.exception.toString());
    }

    return result.data?['toggleSpotLike'] ?? false;
  }

  static Future<List<Comment>> fetchTrickComments(String trickId) async {
    const String getCommentsQuery = r'''
      query GetComments($trickId: UUID!) {
        getTrickComments(trickId: $trickId) {
          id
          userId
          trickId
          content
          createdAt
        }
      }
    ''';

    final client = await _getClient();
    final QueryOptions options = QueryOptions(
      document: gql(getCommentsQuery),
      variables: {'trickId': trickId},
      fetchPolicy: FetchPolicy.networkOnly,
    );

    final QueryResult result = await client.query(options);

    if (result.hasException) {
      throw Exception(result.exception.toString());
    }

    final List<dynamic> commentsJson = result.data?['getTrickComments'] ?? [];
    return commentsJson.map((json) => Comment.fromJson(json)).toList();
  }

  static Future<List<Comment>> fetchSpotComments(String spotId) async {
    const String getSpotCommentsQuery = r'''
      query GetSpotComments($spotId: UUID!) {
        getSpotComments(spotId: $spotId) {
          id
          userId
          trickId
          content
          createdAt
        }
      }
    ''';

    final client = await _getClient();
    final QueryOptions options = QueryOptions(
      document: gql(getSpotCommentsQuery),
      variables: {'spotId': spotId},
      fetchPolicy: FetchPolicy.networkOnly,
    );

    final QueryResult result = await client.query(options);

    if (result.hasException) {
      throw Exception(result.exception.toString());
    }

    final List<dynamic> commentsJson = result.data?['getSpotComments'] ?? [];
    return commentsJson.map((json) => Comment.fromJson(json)).toList();
  }

  static Future<Comment> addSpotComment(String spotId, String content) async {
    const String addSpotCommentMutation = r'''
      mutation AddSpotComment($spotId: UUID!, $content: String!) {
        addSpotComment(spotId: $spotId, content: $content) {
          id
          userId
          trickId
          content
          createdAt
        }
      }
    ''';

    final client = await _getClient();
    final MutationOptions options = MutationOptions(
      document: gql(addSpotCommentMutation),
      variables: {
        'spotId': spotId,
        'content': content,
      },
    );

    final QueryResult result = await client.mutate(options);

    if (result.hasException) {
      throw Exception(result.exception.toString());
    }

    final data = result.data?['addSpotComment'];
    if (data == null) throw Exception("Failed to add spot comment");

    return Comment.fromJson(data);
  }

  static Future<Comment> addComment(String trickId, String content) async {
    const String addCommentMutation = r'''
      mutation AddComment($trickId: UUID!, $content: String!) {
        addComment(trickId: $trickId, content: $content) {
          id
          userId
          trickId
          content
          createdAt
        }
      }
    ''';

    final client = await _getClient();
    final MutationOptions options = MutationOptions(
      document: gql(addCommentMutation),
      variables: {
        'trickId': trickId,
        'content': content,
      },
    );

    final QueryResult result = await client.mutate(options);

    if (result.hasException) {
      throw Exception(result.exception.toString());
    }

    final data = result.data?['addComment'];
    if (data == null) throw Exception("Failed to add comment");

    return Comment.fromJson(data);
  }

  static Future<bool> deleteComment(String commentId) async {
    const String deleteCommentMutation = r'''
      mutation DeleteComment($commentId: UUID!) {
        deleteComment(commentId: $commentId)
      }
    ''';

    final client = await _getClient();
    final MutationOptions options = MutationOptions(
      document: gql(deleteCommentMutation),
      variables: {'commentId': commentId},
    );

    final QueryResult result = await client.mutate(options);
    return result.data?['deleteComment'] ?? false;
  }

  static Future<bool> deleteSpotComment(String commentId) async {
    const String deleteSpotCommentMutation = r'''
      mutation DeleteSpotComment($commentId: UUID!) {
        deleteSpotComment(commentId: $commentId)
      }
    ''';

    final client = await _getClient();
    final MutationOptions options = MutationOptions(
      document: gql(deleteSpotCommentMutation),
      variables: {'commentId': commentId},
    );

    final QueryResult result = await client.mutate(options);
    return result.data?['deleteSpotComment'] ?? false;
  }
}
