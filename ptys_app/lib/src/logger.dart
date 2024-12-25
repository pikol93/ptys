import "package:flutter/foundation.dart";
import "package:loggy/loggy.dart";

class LoggerPrinter extends LoggyPrinter {
  const LoggerPrinter();

  static final Map<LogLevel, String> _levelStrings = <LogLevel, String>{
    LogLevel.debug: "DEBUG",
    LogLevel.info: "INFO ",
    LogLevel.warning: "WARN ",
    LogLevel.error: "ERROR",
  };

  @override
  void onLog(LogRecord record) {
    if (kDebugMode) {
      final time = record.time.toIso8601String().split("T")[1];
      final logLevel = _levelStrings[record.level]!;
      print("$time $logLevel ${record.loggerName} ${record.message}");
    }
  }
}

mixin Logger implements LoggyType {
  Loggy<Logger> get logger => loggy;

  @override
  Loggy<Logger> get loggy => Loggy<Logger>(runtimeType.toString());
}