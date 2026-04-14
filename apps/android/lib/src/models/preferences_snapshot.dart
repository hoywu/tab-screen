import 'connection_mode.dart';

class PreferencesSnapshot {
  const PreferencesSnapshot({
    required this.clientStableId,
    required this.lastSuccessfulServer,
    required this.preferredConnectionMode,
  });

  final String clientStableId;
  final String? lastSuccessfulServer;
  final ConnectionMode preferredConnectionMode;
}
