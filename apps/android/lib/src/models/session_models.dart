import 'connection_mode.dart';

enum SessionViewState { idle, connecting, connected, streaming, error }

class SessionStatus {
  const SessionStatus({
    required this.state,
    required this.connectionMode,
    this.serverAddress,
    this.detail,
  });

  final SessionViewState state;
  final ConnectionMode connectionMode;
  final String? serverAddress;
  final String? detail;

  SessionStatus copyWith({
    SessionViewState? state,
    ConnectionMode? connectionMode,
    String? serverAddress,
    String? detail,
  }) {
    return SessionStatus(
      state: state ?? this.state,
      connectionMode: connectionMode ?? this.connectionMode,
      serverAddress: serverAddress ?? this.serverAddress,
      detail: detail ?? this.detail,
    );
  }
}

class SettingsState {
  const SettingsState({
    required this.followServerPreference,
    required this.preferHardwareDecode,
    this.requestedServerAddress,
  });

  final bool followServerPreference;
  final bool preferHardwareDecode;
  final String? requestedServerAddress;

  SettingsState copyWith({
    bool? followServerPreference,
    bool? preferHardwareDecode,
    String? requestedServerAddress,
  }) {
    return SettingsState(
      followServerPreference:
          followServerPreference ?? this.followServerPreference,
      preferHardwareDecode: preferHardwareDecode ?? this.preferHardwareDecode,
      requestedServerAddress:
          requestedServerAddress ?? this.requestedServerAddress,
    );
  }
}

class DiagnosticsState {
  const DiagnosticsState({
    required this.connectionMode,
    required this.decoderBackend,
    this.lastError,
  });

  final ConnectionMode connectionMode;
  final String decoderBackend;
  final String? lastError;

  DiagnosticsState copyWith({
    ConnectionMode? connectionMode,
    String? decoderBackend,
    String? lastError,
  }) {
    return DiagnosticsState(
      connectionMode: connectionMode ?? this.connectionMode,
      decoderBackend: decoderBackend ?? this.decoderBackend,
      lastError: lastError ?? this.lastError,
    );
  }
}
