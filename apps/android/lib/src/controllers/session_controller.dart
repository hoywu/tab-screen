import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../models/connection_mode.dart';
import '../models/session_models.dart';
import '../repositories/preferences_repository.dart';

final preferencesRepositoryProvider = Provider<PreferencesRepository>((ref) {
  return PreferencesRepository();
});

final sessionControllerProvider =
    NotifierProvider<SessionController, SessionStatus>(SessionController.new);

class SessionController extends Notifier<SessionStatus> {
  @override
  SessionStatus build() {
    return const SessionStatus(
      state: SessionViewState.idle,
      connectionMode: ConnectionMode.lan,
      detail: 'Phase 0 session controller placeholder',
    );
  }

  Future<void> loadBootstrapState() async {
    final snapshot = await ref.read(preferencesRepositoryProvider).load();
    state = state.copyWith(
      connectionMode: snapshot.preferredConnectionMode,
      serverAddress: snapshot.lastSuccessfulServer,
      detail: 'Loaded local stable id and last successful server',
    );
  }

  Future<void> rememberLastServer(String address) async {
    await ref
        .read(preferencesRepositoryProvider)
        .saveLastSuccessfulServer(address);
    state = state.copyWith(serverAddress: address);
  }
}
