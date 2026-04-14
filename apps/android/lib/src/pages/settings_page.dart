import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../controllers/settings_controller.dart';
import '../widgets/page_shell.dart';

class SettingsPage extends ConsumerWidget {
  const SettingsPage({super.key});

  static const routeName = '/settings';

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final settings = ref.watch(settingsControllerProvider);
    final controller = ref.read(settingsControllerProvider.notifier);

    return PageShell(
      title: 'Settings',
      subtitle:
          'Phase 0 keeps only the key toggles required by the architecture skeleton.',
      children: [
        SwitchListTile(
          title: const Text('Follow server preference'),
          value: settings.followServerPreference,
          onChanged: controller.setFollowServerPreference,
        ),
        SwitchListTile(
          title: const Text('Prefer hardware decode'),
          value: settings.preferHardwareDecode,
          onChanged: controller.setPreferHardwareDecode,
        ),
      ],
    );
  }
}
