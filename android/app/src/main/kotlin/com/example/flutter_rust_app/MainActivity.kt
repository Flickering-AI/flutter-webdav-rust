package com.example.flutter_rust_app

import android.content.Context
import android.os.Bundle
import android.os.PersistableBundle
import androidx.annotation.NonNull
import expo.modules.webdavserver.ExpoWebdavServerModule
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel
import java.io.File

class MainActivity: FlutterActivity() {

    private val CHANNEL = "samples.flutter.dev/battery"

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)
        MethodChannel(flutterEngine.dartExecutor.binaryMessenger, CHANNEL).setMethodCallHandler {
                call, result ->
            ExpoWebdavServerModule().testJNI(filesDir.absolutePath)
        }
    }
}
