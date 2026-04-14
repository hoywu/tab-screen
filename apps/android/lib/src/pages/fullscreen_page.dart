import 'package:flutter/material.dart';

import '../widgets/page_shell.dart';

class FullscreenPage extends StatelessWidget {
  const FullscreenPage({super.key});

  static const routeName = '/fullscreen';

  @override
  Widget build(BuildContext context) {
    return const PageShell(
      title: 'Fullscreen',
      subtitle:
          'Phase 2 will replace this placeholder with the decoder-backed immersive renderer.',
      children: [
        AspectRatio(
          aspectRatio: 16 / 10,
          child: DecoratedBox(
            decoration: BoxDecoration(
              gradient: LinearGradient(
                colors: [Color(0xFF111827), Color(0xFF312E81)],
              ),
              borderRadius: BorderRadius.all(Radius.circular(24)),
            ),
            child: Center(
              child: Text(
                'Video surface placeholder',
                style: TextStyle(fontSize: 18),
              ),
            ),
          ),
        ),
      ],
    );
  }
}
