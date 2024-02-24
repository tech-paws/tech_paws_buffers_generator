sealed interface MyEnumWithoutPositions {
    companion object {
        fun createDefault(): MyEnumWithoutPositions = MyEnumWithoutPositionsOption1(
            0UL,
        )
    }
}

data class MyEnumWithoutPositionsOption1(
    val p0: ULong,
) : MyEnumWithoutPositions

data class MyEnumWithoutPositionsOption2(
    val name: String,
) : MyEnumWithoutPositions

object MyEnumWithoutPositionsOption3 : MyEnumWithoutPositions

object MyEnumWithoutPositionsOption4 : MyEnumWithoutPositions
