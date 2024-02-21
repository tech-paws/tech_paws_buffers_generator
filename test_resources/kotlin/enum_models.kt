sealed interface MyEnum {
    companion object {
        fun createDefault(): MyEnum = MyEnumIdle
    }
}

object MyEnumIdle : MyEnum

data class MyEnumMove(
    val x: Float,
    val y: Float,
) : MyEnum

data class MyEnumUpdate(
    val p0: Float,
    val p1: Float,
    val p2: String,
) : MyEnum

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
