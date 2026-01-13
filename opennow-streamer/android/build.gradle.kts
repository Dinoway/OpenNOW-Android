plugins {
    id("com.android.application") version "8.2.0"
}

android {
    namespace = "com.opennow.streamer"
    compileSdk = 34
    ndkVersion = "26.1.10909125"

    defaultConfig {
        applicationId = "com.opennow.streamer"
        minSdk = 29  // Android 10 for AHardwareBuffer
        targetSdk = 34
        versionCode = 1
        versionName = "0.3.0"

        ndk {
            abiFilters.addAll(listOf("arm64-v8a", "x86_64"))
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            isShrinkResources = false
        }
    }

    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("jniLibs")
        }
    }
}