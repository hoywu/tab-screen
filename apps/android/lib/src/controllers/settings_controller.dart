import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../models/session_models.dart';

final settingsControllerProvider =
    NotifierProvider<SettingsController, SettingsState>(SettingsController.new);

class SettingsController extends Notifier<SettingsState> {
  @override
  SettingsState build() {
    return const SettingsState(
      followServerPreference: true,
      preferHardwareDecode: true,
    );
  }

  void setFollowServerPreference(bool value) {
    state = state.copyWith(followServerPreference: value);
  }

  void setPreferHardwareDecode(bool value) {
    state = state.copyWith(preferHardwareDecode: value);
  }
}
