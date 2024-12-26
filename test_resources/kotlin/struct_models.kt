class Empty() {
    companion object {
        fun createDefault(): Empty = Empty()

        fun readFromBuffers(reader: Long): Empty {
            return Empty()
        }

        fun skipInBuffers(reader: Long, count: Int) {
        }
    }

    fun writeToBuffers(writer: Long) {
    }
}

data class ViewData(
    val deltaTime: Float,
    val viewWidth: Float,
    val viewHeight: Float,
    val touchStartX: Float,
    val touchStartY: Float,
    val lastTouchX: Float,
    val lastTouchY: Float,
    val touchX: Float,
    val touchY: Float,
) {
    companion object {
        fun createDefault(): ViewData = ViewData(
            deltaTime = 0f,
            viewWidth = 0f,
            viewHeight = 0f,
            touchStartX = 0f,
            touchStartY = 0f,
            lastTouchX = 0f,
            lastTouchY = 0f,
            touchX = 0f,
            touchY = 0f,
        )

        fun readFromBuffers(reader: Long): ViewData {
            val deltaTime = Float.readFromBuffers(reader)
            val viewWidth = Float.readFromBuffers(reader)
            val viewHeight = Float.readFromBuffers(reader)
            val touchStartX = Float.readFromBuffers(reader)
            val touchStartY = Float.readFromBuffers(reader)
            val lastTouchX = Float.readFromBuffers(reader)
            val lastTouchY = Float.readFromBuffers(reader)
            val touchX = Float.readFromBuffers(reader)
            val touchY = Float.readFromBuffers(reader)

            return ViewData(
                deltaTime = deltaTime,
                viewWidth = viewWidth,
                viewHeight = viewHeight,
                touchStartX = touchStartX,
                touchStartY = touchStartY,
                lastTouchX = lastTouchX,
                lastTouchY = lastTouchY,
                touchX = touchX,
                touchY = touchY,
            )
        }

        fun skipInBuffers(reader: Long, count: Int) {
            for (i in 0..<count) {
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
            }
        }
    }

    fun writeToBuffers(writer: Long) {
        deltaTime.writeToBuffers(writer)
        viewWidth.writeToBuffers(writer)
        viewHeight.writeToBuffers(writer)
        touchStartX.writeToBuffers(writer)
        touchStartY.writeToBuffers(writer)
        lastTouchX.writeToBuffers(writer)
        lastTouchY.writeToBuffers(writer)
        touchX.writeToBuffers(writer)
        touchY.writeToBuffers(writer)
    }
}

data class Test(
    val touchY: Float,
    val touchX: Float,
    val touchStatus: TouchStatus,
) {
    companion object {
        fun createDefault(): Test = Test(
            touchY = 0f,
            touchX = 0f,
            touchStatus = TouchStatus.createDefault(),
        )

        fun readFromBuffers(reader: Long): Test {
            val touchY = Float.readFromBuffers(reader)
            val touchX = Float.readFromBuffers(reader)
            val touchStatus = TouchStatus.readFromBuffers(reader)

            return Test(
                touchY = touchY,
                touchX = touchX,
                touchStatus = touchStatus,
            )
        }

        fun skipInBuffers(reader: Long, count: Int) {
            for (i in 0..<count) {
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                TouchStatus.readFromBuffers(reader)
            }
        }
    }

    fun writeToBuffers(writer: Long) {
        touchY.writeToBuffers(writer)
        touchX.writeToBuffers(writer)
        touchStatus.writeToBuffers(writer)
    }
}

data class GenericType(
    val items: List<Test>,
    val table: LinearTable<Float, Test>,
) {
    companion object {
        fun createDefault(): GenericType = GenericType(
            items = listOf(),
            table = LinearTable.createDefault<Float, Test>(),
        )

        fun readFromBuffers(reader: Long): GenericType {
            val items = readFromBuffersList(reader) {
                Test.readFromBuffers(reader)
            }
            val table = LinearTable<Float, Test>.readFromBuffers(reader)

            return GenericType(
                items = items,
                table = table,
            )
        }

        fun skipInBuffers(reader: Long, count: Int) {
            for (i in 0..<count) {
                readFromBuffersList(reader) {
                    Test.readFromBuffers(reader)
                }
                LinearTable<Float, Test>.readFromBuffers(reader)
            }
        }
    }

    fun writeToBuffers(writer: Long) {
        writeToBuffersList(writer, items) { itemsItem ->
            itemsItem.writeToBuffers(writer)
        }
        table.writeToBuffers(writer)
    }
}
