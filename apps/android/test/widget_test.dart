import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:tab_screen/src/app.dart';
import 'package:tab_screen/src/models/connection_mode.dart';
import 'package:tab_screen/src/repositories/preferences_repository.dart';

void main() {
  testWidgets('app shows phase 0 home shell', (tester) async {
    await tester.pumpWidget(const ProviderScope(child: TabScreenApp()));
    await tester.pumpAndSettle();

    expect(find.text('Tab Screen'), findsWidgets);
    expect(find.text('Open Connect'), findsOneWidget);
    expect(find.text('Fullscreen'), findsOneWidget);
    expect(find.text('Settings'), findsOneWidget);
    expect(find.text('Diagnostics'), findsOneWidget);
  });

  test('preferences repository loads stable id and saved mode', () async {
    SharedPreferences.setMockInitialValues({
      PreferencesRepository.clientStableIdKey: 'client-123',
      PreferencesRepository.lastSuccessfulServerKey: '127.0.0.1:38491',
      PreferencesRepository.preferredConnectionModeKey: ConnectionMode.usb.name,
    });

    final repository = PreferencesRepository();
    final snapshot = await repository.load();

    expect(snapshot.clientStableId, 'client-123');
    expect(snapshot.lastSuccessfulServer, '127.0.0.1:38491');
    expect(snapshot.preferredConnectionMode, ConnectionMode.usb);
  });
}
