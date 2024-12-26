extension IntExtension on int {
  bool inRangeInclusive(int min, int max) {
    return this >= min && this <= max;
  }
}
