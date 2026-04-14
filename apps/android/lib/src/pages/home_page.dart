import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../controllers/session_controller.dart';
import '../widgets/page_shell.dart';
import 'connect_page.dart';
import 'diagnostics_page.dart';
import 'fullscreen_page.dart';
import 'settings_page.dart';

class HomePage extends ConsumerStatefulWidget {
  const HomePage({super.key});

  static const routeName = '/';

  @override
  ConsumerState<HomePage> createState() => _HomePageState();
}

class _HomePageState extends ConsumerState<HomePage> {
  @override
  void initState() {
    super.initState();
    Future.microtask(
      () => ref.read(sessionControllerProvider.notifier).loadBootstrapState(),
    );
  }

  @override
  Widget build(BuildContext context) {
    final session = ref.watch(sessionControllerProvider);

    return PageShell(
      title: 'Tab Screen',
      subtitle:
          'Phase 0 Android shell with route, controller, and storage placeholders.',
      children: [
        Card(
          child: ListTile(
            title: const Text('Session state'),
            subtitle: Text(
              '${session.state.name} · ${session.detail ?? 'No detail'}',
            ),
            trailing: Text(session.connectionMode.name.toUpperCase()),
          ),
        ),
        Card(
          child: ListTile(
            title: const Text('Last successful server'),
            subtitle: Text(session.serverAddress ?? 'Not recorded yet'),
          ),
        ),
        const SizedBox(height: 12),
        FilledButton(
          onPressed: () => Navigator.pushNamed(context, ConnectPage.routeName),
          child: const Text('Open Connect'),
        ),
        const SizedBox(height: 12),
        Wrap(
          spacing: 12,
          runSpacing: 12,
          children: [
            _NavChip(label: 'Fullscreen', routeName: FullscreenPage.routeName),
            _NavChip(label: 'Settings', routeName: SettingsPage.routeName),
            _NavChip(
              label: 'Diagnostics',
              routeName: DiagnosticsPage.routeName,
            ),
          ],
        ),
      ],
    );
  }
}

class _NavChip extends StatelessWidget {
  const _NavChip({required this.label, required this.routeName});

  final String label;
  final String routeName;

  @override
  Widget build(BuildContext context) {
    return ActionChip(
      label: Text(label),
      onPressed: () => Navigator.pushNamed(context, routeName),
    );
  }
}
