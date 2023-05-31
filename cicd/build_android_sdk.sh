# Set the ${ANDROID_HOME} variable, so that the tools can find our installation.
# See https://developer.android.com/studio/command-line/variables#envar.
export ANDROID_HOME=${HOME}/android

# Download and extract the command-line tools into ${ANDROID_HOME}.
rm -rf ${ANDROID_HOME}
mkdir -p ${ANDROID_HOME}

# Download and extract the command-line tools into ${ANDROID_HOME}.
mkdir -p ${ANDROID_HOME}
wget https://dl.google.com/android/repository/commandlinetools-linux-8512546_latest.zip \
    -O ${HOME}/commandlinetools-linux-8512546_latest.zip &&
    sha256sum commandlinetools-linux-8512546_latest.zip &&
    echo "2ccbda4302db862a28ada25aa7425d99dce9462046003c1714b059b5c47970d8 commandlinetools-linux-8512546_latest.zip" | sha256sum -c - &&
    unzip commandlinetools-linux-8512546_latest.zip -d ${ANDROID_HOME}/cmdline-tools &&
    rm commandlinetools-linux-8512546_latest.zip

# Add the relevant directories to the $PATH.
export PATH=${PATH}:${ANDROID_HOME}/cmdline-tools/cmdline-tools/bin:${ANDROID_HOME}/platform-tools

# Once these tools are installed, the sdkmanager program is available to install the specific parts of the SDK that we actually need.
# The first step is to accept the software licenses, by answering “yes” to sdkmanager --licenses.
# We then install the build tools for Android SDK 33, as well as the native development kit (NDK) which allows implementing part of the app in a native programming language – in our case Rust.
yes | sdkmanager --licenses &&
    sdkmanager --verbose \
        "build-tools;30.0.3" \
        "ndk;25.1.8937393" \
        "platforms;android-33"
export NDK_HOME=${ANDROID_HOME}/ndk/25.1.8937393
export PATH=$PATH:${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin
