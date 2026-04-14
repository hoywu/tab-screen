import 'package:shared_preferences/shared_preferences.dart';
import 'package:uuid/uuid.dart';

import '../models/connection_mode.dart';
import '../models/preferences_snapshot.dart';

class PreferencesRepository {
  PreferencesRepository({SharedPreferences? sharedPreferences})
    : _sharedPreferences = sharedPreferences,
      _uuid = const Uuid();

  static const clientStableIdKey = 'client_stable_id';
  static const lastSuccessfulServerKey = 'last_successful_server';
  static const preferredConnectionModeKey = 'preferred_connection_mode';

  SharedPreferences? _sharedPreferences;
  final Uuid _uuid;

  Future<SharedPreferences> _prefs() async {
    return _sharedPreferences ??= await SharedPreferences.getInstance();
  }

  // Phase 0 persistence covers only the stable identity and last server handoff path.
  Future<PreferencesSnapshot> load() async {
    final prefs = await _prefs();
    final stableId = prefs.getString(clientStableIdKey) ?? _uuid.v4();
    if (!prefs.containsKey(clientStableIdKey)) {
      await prefs.setString(clientStableIdKey, stableId);
    }

    return PreferencesSnapshot(
      clientStableId: stableId,
      lastSuccessfulServer: prefs.getString(lastSuccessfulServerKey),
      preferredConnectionMode: _parseMode(
        prefs.getString(preferredConnectionModeKey),
      ),
    );
  }

  Future<void> saveLastSuccessfulServer(String serverAddress) async {
    final prefs = await _prefs();
    await prefs.setString(lastSuccessfulServerKey, serverAddress);
  }

  Future<void> savePreferredConnectionMode(ConnectionMode mode) async {
    final prefs = await _prefs();
    await prefs.setString(preferredConnectionModeKey, mode.name);
  }

  ConnectionMode _parseMode(String? value) {
    return ConnectionMode.values.firstWhere(
      (mode) => mode.name == value,
      orElse: () => ConnectionMode.lan,
    );
  }
}
