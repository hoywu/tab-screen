import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../controllers/diagnostics_controller.dart';
import '../widgets/page_shell.dart';

class DiagnosticsPage extends ConsumerWidget {
  const DiagnosticsPage({super.key});

  static const routeName = '/diagnostics';

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final diagnostics = ref.watch(diagnosticsControllerProvider);

    return PageShell(
      title: 'Diagnostics',
      subtitle:
          'Runtime metrics and recent failures will land here in later phases.',
      children: [
        Card(
          child: ListTile(
            title: const Text('Connection mode'),
            subtitle: Text(diagnostics.connectionMode.name.toUpperCase()),
          ),
        ),
        Card(
          child: ListTile(
            title: const Text('Decoder backend'),
            subtitle: Text(diagnostics.decoderBackend),
          ),
        ),
        Card(
          child: ListTile(
            title: const Text('Last error'),
            subtitle: Text(diagnostics.lastError ?? 'None'),
          ),
        ),
      ],
    );
  }
}
