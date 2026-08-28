import 'package:flutter/material.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:latlong2/latlong.dart';
import 'package:geolocator/geolocator.dart';
import '../services/auth_service.dart';
import '../services/spot_service.dart';
import 'spot_detail_screen.dart';
import 'add_trick_screen.dart';

class HomeScreen extends StatefulWidget {
  final VoidCallback onLogout;

  const HomeScreen({super.key, required this.onLogout});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  final MapController _mapController = MapController();
  LatLng? _currentPosition;
  List<Spot> _spots = [];
  bool _isLoading = true;
  String _errorMessage = '';

  @override
  void initState() {
    super.initState();
    _initializeData();
  }

  Future<void> _initializeData() async {
    await _determinePosition();
    await _loadSpots();
    setState(() {
      _isLoading = false;
    });
  }

  Future<void> _loadSpots() async {
    try {
      final spots = await SpotService.fetchSpots();
      setState(() {
        _spots = spots;
      });
    } catch (e) {
       debugPrint('Erreur lors du chargement des spots : $e');
    }
  }

  Future<void> _determinePosition() async {
    bool serviceEnabled;
    LocationPermission permission;

    try {
      serviceEnabled = await Geolocator.isLocationServiceEnabled();
      if (!serviceEnabled) {
        setState(() {
          _errorMessage = 'Les services de localisation sont désactivés.';
        });
        return;
      }

      permission = await Geolocator.checkPermission();
      if (permission == LocationPermission.denied) {
        permission = await Geolocator.requestPermission();
        if (permission == LocationPermission.denied) {
          setState(() {
            _errorMessage = 'Les permissions de localisation sont refusées.';
          });
          return;
        }
      }

      if (permission == LocationPermission.deniedForever) {
        setState(() {
          _errorMessage = 'Les permissions sont refusées de façon permanente.';
        });
        return;
      }

      Position position = await Geolocator.getCurrentPosition();
      setState(() {
        _currentPosition = LatLng(position.latitude, position.longitude);
      });
    } catch (e) {
      setState(() {
        _errorMessage = 'Erreur lors de la récupération de la position : $e';
      });
    }
  }

  void _showSpotDetails(Spot spot) {
    showModalBottomSheet(
      context: context,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (context) {
        return Container(
          padding: const EdgeInsets.all(20),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                spot.name ?? 'Spot sans nom',
                style: const TextStyle(fontSize: 22, fontWeight: FontWeight.bold),
              ),
              const SizedBox(height: 10),
              Text(
                'Coordonnées : ${spot.latitude.toStringAsFixed(4)}, ${spot.longitude.toStringAsFixed(4)}',
                style: TextStyle(color: Colors.grey[600]),
              ),
              const SizedBox(height: 20),
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Icon(Icons.favorite, size: 16, color: spot.isLikedByMe ? Colors.redAccent : Colors.grey),
                  const SizedBox(width: 4),
                  Text('${spot.likesCount}', style: const TextStyle(fontWeight: FontWeight.bold)),
                  const SizedBox(width: 20),
                  const Icon(Icons.chat_bubble_outline, size: 16, color: Colors.grey),
                  const SizedBox(width: 4),
                  Text('${spot.commentsCount}', style: const TextStyle(fontWeight: FontWeight.bold)),
                ],
              ),
              const SizedBox(height: 20),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                children: [
                  TextButton(
                    onPressed: () => Navigator.pop(context),
                    child: const Text('ANNULER', style: TextStyle(color: Colors.black38, fontWeight: FontWeight.bold)),
                  ),
                    ElevatedButton(
                    onPressed: () {
                      Navigator.pop(context);
                      Navigator.push(
                        context,
                        MaterialPageRoute(
                          builder: (context) => SpotDetailScreen(spotId: spot.id),
                        ),
                      );
                    },
                    style: ElevatedButton.styleFrom(
                      backgroundColor: const Color(0xFF1A1A1A),
                      foregroundColor: Colors.white,
                    ),
                    child: const Text('VOIR LE SPOT'),
                  ),
                ],
              ),
            ],
          ),
        );
      },
    );
  }

  void _zoomIn() {
    final zoom = _mapController.camera.zoom + 1;
    _mapController.move(_mapController.camera.center, zoom);
  }

  void _zoomOut() {
    final zoom = _mapController.camera.zoom - 1;
    _mapController.move(_mapController.camera.center, zoom);
  }

  void _recenter() {
    if (_currentPosition != null) {
      _mapController.move(_currentPosition!, 15.0);
    }
  }

  Future<void> _navigateToAddTrick() async {
    if (_currentPosition == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Position GPS inconnue')),
      );
      return;
    }

    final result = await Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => AddTrickScreen(initialLocation: _currentPosition!),
      ),
    );

    if (result == true) {
      _loadSpots();
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Accueil Sesh'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: _loadSpots,
          ),
          IconButton(
            icon: const Icon(Icons.logout),
            onPressed: () async {
              await AuthService.logout();
              widget.onLogout();
            },
          ),
        ],
      ),
      body: _isLoading
          ? const Center(child: CircularProgressIndicator())
          : Stack(
              children: [
                FlutterMap(
                  mapController: _mapController,
                  options: MapOptions(
                    initialCenter: _currentPosition ?? const LatLng(48.8566, 2.3522),
                    initialZoom: 13.0,
                    interactionOptions: const InteractionOptions(
                      flags: InteractiveFlag.all,
                    ),
                  ),
                  children: [
                    TileLayer(
                      urlTemplate: 'https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}',
                      userAgentPackageName: 'com.sesh.app',
                    ),
                    // Marqueurs des spots simplifiés
                    MarkerLayer(
                      markers: _spots.map((spot) => Marker(
                        point: spot.location,
                        width: 30,
                        height: 30,
                        child: GestureDetector(
                          onTap: () => _showSpotDetails(spot),
                          child: Container(
                            decoration: BoxDecoration(
                              color: Colors.white,
                              shape: BoxShape.circle,
                              border: Border.all(color: const Color(0xFF1A1A1A), width: 3),
                              boxShadow: [
                                BoxShadow(
                                  color: Colors.black.withValues(alpha: 0.3),
                                  blurRadius: 10,
                                  offset: const Offset(0, 4),
                                ),
                              ],
                            ),
                            child: const Center(
                              child: Icon(Icons.skateboarding, size: 15, color: Color(0xFF1A1A1A)),
                            ),
                          ),
                        ),
                      )).toList(),
                    ),
                    if (_currentPosition != null)
                      MarkerLayer(
                        markers: [
                          Marker(
                            point: _currentPosition!,
                            width: 20,
                            height: 20,
                            child: Container(
                              decoration: BoxDecoration(
                                color: Colors.blueAccent,
                                shape: BoxShape.circle,
                                border: Border.all(color: Colors.white, width: 3),
                              ),
                            ),
                          ),
                        ],
                      ),
                  ],
                ),
                Positioned(
                  left: 0,
                  right: 0,
                  bottom: 30,
                  child: Center(
                    child: FloatingActionButton.extended(
                      heroTag: 'addTrick',
                      onPressed: _navigateToAddTrick,
                      backgroundColor: const Color(0xFF1A1A1A),
                      foregroundColor: Colors.white,
                      icon: const Icon(Icons.add),
                      label: const Text('AJOUTER UN TRICK', style: TextStyle(fontWeight: FontWeight.w900, letterSpacing: -0.5)),
                      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20)),
                    ),
                  ),
                ),
                Positioned(
                  right: 16,
                  bottom: 100,
                  child: Column(
                    children: [
                      FloatingActionButton(
                        heroTag: 'recenter',
                        mini: true,
                        onPressed: _recenter,
                        backgroundColor: Colors.white,
                        foregroundColor: const Color(0xFF1A1A1A),
                        child: const Icon(Icons.gps_fixed),
                      ),
                      const SizedBox(height: 8),
                      FloatingActionButton(
                        heroTag: 'zoomIn',
                        mini: true,
                        onPressed: _zoomIn,
                        backgroundColor: Colors.white,
                        foregroundColor: const Color(0xFF1A1A1A),
                        child: const Icon(Icons.add),
                      ),
                      const SizedBox(height: 8),
                      FloatingActionButton(
                        heroTag: 'zoomOut',
                        mini: true,
                        onPressed: _zoomOut,
                        backgroundColor: Colors.white,
                        foregroundColor: const Color(0xFF1A1A1A),
                        child: const Icon(Icons.remove),
                      ),
                    ],
                  ),
                ),
                if (_errorMessage.isNotEmpty)
                  Positioned(
                    top: 20,
                    left: 20,
                    right: 20,
                    child: Container(
                      padding: const EdgeInsets.all(10),
                      decoration: BoxDecoration(
                        color: Colors.red.withValues(alpha: 0.8),
                        borderRadius: BorderRadius.circular(8),
                      ),
                      child: Text(
                        _errorMessage,
                        style: const TextStyle(color: Colors.white),
                        textAlign: TextAlign.center,
                      ),
                    ),
                  ),
              ],
            ),
    );
  }
}
