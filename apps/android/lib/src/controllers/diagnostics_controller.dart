import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../models/connection_mode.dart';
import '../models/session_models.dart';

final diagnosticsControllerProvider =
    NotifierProvider<DiagnosticsController, DiagnosticsState>(
      DiagnosticsController.new,
    );

class DiagnosticsController extends Notifier<DiagnosticsState> {
  @override
  DiagnosticsState build() {
    return const DiagnosticsState(
      connectionMode: ConnectionMode.lan,
      decoderBackend: 'native-plugin-placeholder',
      lastError: null,
    );
  }

  void reportError(String message) {
    state = state.copyWith(lastError: message);
  }
}
