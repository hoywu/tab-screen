import 'package:flutter/material.dart';

import 'pages/connect_page.dart';
import 'pages/diagnostics_page.dart';
import 'pages/fullscreen_page.dart';
import 'pages/home_page.dart';
import 'pages/settings_page.dart';

class TabScreenApp extends StatelessWidget {
  const TabScreenApp({super.key});

  @override
  Widget build(BuildContext context) {
    final colorScheme = ColorScheme.fromSeed(
      seedColor: const Color(0xFF4F46E5),
      brightness: Brightness.dark,
    );

    return MaterialApp(
      title: 'Tab Screen',
      debugShowCheckedModeBanner: false,
      themeMode: ThemeMode.system,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xFF4F46E5)),
        useMaterial3: true,
      ),
      darkTheme: ThemeData(
        colorScheme: colorScheme,
        useMaterial3: true,
        scaffoldBackgroundColor: const Color(0xFF0B1020),
      ),
      initialRoute: HomePage.routeName,
      routes: {
        HomePage.routeName: (_) => const HomePage(),
        ConnectPage.routeName: (_) => const ConnectPage(),
        FullscreenPage.routeName: (_) => const FullscreenPage(),
        SettingsPage.routeName: (_) => const SettingsPage(),
        DiagnosticsPage.routeName: (_) => const DiagnosticsPage(),
      },
    );
  }
}
