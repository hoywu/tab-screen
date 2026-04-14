import 'package:flutter/services.dart';

class NativeDecoderPlugin {
  static const MethodChannel _channel = MethodChannel(
    'dev.tabscreen/native_decoder',
  );

  Future<int> createRenderer() async {
    final textureId = await _channel.invokeMethod<int>('createRenderer');
    return textureId ?? -1;
  }

  Future<void> initializeDecoder({
    required String codec,
    required int width,
    required int height,
    List<int> configBytes = const <int>[],
  }) {
    return _channel.invokeMethod<void>('initializeDecoder', {
      'codec': codec,
      'width': width,
      'height': height,
      'configBytes': configBytes,
    });
  }

  Future<void> queueAccessUnit({
    required List<int> bytes,
    required int ptsUs,
    required bool isKeyFrame,
  }) {
    return _channel.invokeMethod<void>('queueAccessUnit', {
      'bytes': bytes,
      'ptsUs': ptsUs,
      'isKeyFrame': isKeyFrame,
    });
  }

  Future<void> reconfigure({
    required String codec,
    required int width,
    required int height,
    List<int> configBytes = const <int>[],
  }) {
    return _channel.invokeMethod<void>('reconfigure', {
      'codec': codec,
      'width': width,
      'height': height,
      'configBytes': configBytes,
    });
  }

  Future<void> releaseSession() {
    return _channel.invokeMethod<void>('releaseSession');
  }
}
