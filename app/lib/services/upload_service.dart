import 'dart:io';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'package:image_picker/image_picker.dart';
import 'package:video_compress/video_compress.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';

class UploadService {
  static String get _cloudName => dotenv.get('CLOUDINARY_CLOUD_NAME');
  static String get _uploadPreset => dotenv.get('CLOUDINARY_UPLOAD_PRESET');

  static final ImagePicker _picker = ImagePicker();

  static Future<String?> pickAndUploadVideo({
    required Function(double) onProgress,
  }) async {
    final XFile? video = await _picker.pickVideo(source: ImageSource.gallery);
    if (video == null) return null;

    MediaInfo? mediaInfo = await VideoCompress.compressVideo(
      video.path,
      quality: VideoQuality.MediumQuality,
      deleteOrigin: false,
    );

    if (mediaInfo == null || mediaInfo.path == null) return null;

    final File fileToUpload = File(mediaInfo.path!);

    final url = Uri.parse("https://api.cloudinary.com/v1_1/$_cloudName/video/upload");
    
    final request = http.MultipartRequest("POST", url)
      ..fields['upload_preset'] = _uploadPreset
      ..files.add(await http.MultipartFile.fromPath('file', fileToUpload.path));

    final response = await request.send();

    if (response.statusCode == 200) {
      final responseData = await response.stream.toBytes();
      final responseString = String.fromCharCodes(responseData);
      final json = jsonDecode(responseString);

      await VideoCompress.deleteAllCache();
      
      return json['secure_url']; 
    } else {
      await VideoCompress.deleteAllCache();
      throw Exception("Erreur lors de l'envoi de la vidéo");
    }
  }
}
