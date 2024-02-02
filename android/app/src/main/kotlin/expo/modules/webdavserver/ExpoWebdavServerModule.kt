package expo.modules.webdavserver

class ExpoWebdavServerModule {

    external fun test(a: Int, b: Int): Int
    external fun webdav(dir: String): String
    external fun asyncComputation(callback: ExpoWebdavServerModule)

    fun testJNI(dir: String) {
        System.loadLibrary("rust_lib")
        println("=====================================================")
        println(test(6, 6))
        println(webdav(dir))
    }

    fun asyncCallback(progress: Int) {
        println("asyncCallback: thread id = " + Thread.currentThread().id + ", progress = " + progress + "%")
    }
}
