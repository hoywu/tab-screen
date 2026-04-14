import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../controllers/session_controller.dart';
import '../widgets/page_shell.dart';

class ConnectPage extends ConsumerStatefulWidget {
  const ConnectPage({super.key});

  static const routeName = '/connect';

  @override
  ConsumerState<ConnectPage> createState() => _ConnectPageState();
}

class _ConnectPageState extends ConsumerState<ConnectPage> {
  final TextEditingController _addressController = TextEditingController();

  @override
  void dispose() {
    _addressController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final session = ref.watch(sessionControllerProvider);

    return PageShell(
      title: 'Connect',
      subtitle:
          'Manual LAN address entry placeholder for the Phase 2 connection flow.',
      children: [
        TextField(
          controller: _addressController,
          decoration: const InputDecoration(
            labelText: 'Server address',
            hintText: '192.168.1.10:38491',
          ),
        ),
        const SizedBox(height: 12),
        FilledButton(
          onPressed: () async {
            final address = _addressController.text.trim();
            if (address.isEmpty) {
              return;
            }

            await ref
                .read(sessionControllerProvider.notifier)
                .rememberLastServer(address);

            if (!context.mounted) {
              return;
            }

            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text('Stored placeholder server address: $address'),
              ),
            );
          },
          child: const Text('Remember Address'),
        ),
        const SizedBox(height: 12),
        Text('Current stored server: ${session.serverAddress ?? 'none'}'),
      ],
    );
  }
}
