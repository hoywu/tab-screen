package dev.tabscreen.tab_screen

import io.flutter.embedding.engine.FlutterEngine
import io.flutter.embedding.android.FlutterActivity
import io.flutter.plugin.common.MethodChannel

class MainActivity : FlutterActivity() {
    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)

        MethodChannel(
            flutterEngine.dartExecutor.binaryMessenger,
            "dev.tabscreen/native_decoder"
        ).setMethodCallHandler { call, result ->
            when (call.method) {
                // Phase 0 exposes the decoder API shape without implementing MediaCodec yet.
                "createRenderer" -> result.success(-1)
                "initializeDecoder",
                "queueAccessUnit",
                "reconfigure",
                "releaseSession" -> result.success(null)
                else -> result.notImplemented()
            }
        }
    }
}
