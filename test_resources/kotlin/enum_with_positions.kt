sealed interface MyEnum {
    companion object {
        fun createDefault(): MyEnum = MyEnumIdle
    }
}

object MyEnumIdle : MyEnum

data class MyEnumMove(
    val x: Double,
    val y: Double,
) : MyEnum

data class MyEnumUpdate(
    val p0: Double,
    val p1: Double,
    val p2: String,
) : MyEnum
