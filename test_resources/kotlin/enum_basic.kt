sealed interface MyEnumWithoutPositions {
    data class Option1(
        val p0: ULong,
    ) : MyEnumWithoutPositions

    data class Option2(
        val name: String,
    ) : MyEnumWithoutPositions

    data object Option3 : MyEnumWithoutPositions

    data object Option4 : MyEnumWithoutPositions

    companion object {
        fun createDefault(): MyEnumWithoutPositions = Option1(
            0UL,
        )

        fun readFromBuffers(reader: Long): MyEnumWithoutPositions {
            return when (val case = UInt.readFromBuffers(reader)) {
                0U -> {
                    val p0 = ULong.readFromBuffers(reader)

                    Option1(
                        p0 = p0,
                    )
                }
                1U -> {
                    val name = String.readFromBuffers(reader)

                    Option2(
                        name = name,
                    )
                }
                2U -> Option3
                3U -> Option4
                else -> throw IllegalArgumentException("Invalid enum value: $case")
            }
        }

        fun skipInBuffers(reader: Long, count: Int) {
            for (i in 0..<count) {
                readFromBuffers(reader)
            }
        }
    }

    fun writeToBuffers(writer: Long) {
        when (this) {
            is Option1 -> {
                p0.writeToBuffers(writer)
            }
            is Option2 -> {
                name.writeToBuffers(writer)
            }
            Option3 -> {}
            Option4 -> {}
        }
    }
}
