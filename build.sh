#!/bin/bash
cargo clean
rm *.ll *.dylib *.o *.a
wget https://cdn.discordapp.com/attachments/714845075506462820/718298406279315506/librustsupport.dylib

cargo rustc --target=aarch64-apple-ios -- -C embed-bitcode=yes -C lto=fat -C save-temps --emit=llvm-ir
#cargo rustc --target=armv7-apple-ios -- -C embed-bitcode=yes -C lto --emit=llvm-ir
#cargo rustc --target=armv7s-apple-ios -- -C embed-bitcode=yes -C lto --emit=llvm-ir

cp target/aarch64-apple-ios/debug/deps/reachccrust.ll arm64.ll
cp target/aarch64-apple-ios/debug/deps/reachccrust.ll arm64e.ll
#cp target/armv7-apple-ios/debug/deps/reachccrust.ll armv7.ll
#cp target/armv7s-apple-ios/debug/deps/reachccrust.ll armv7s.ll

sed -i "" 's/triple = "arm64-apple-ios/triple = "arm64e-apple-ios/g' arm64e.ll

~/llvm-apple/bin/llc -mcpu=vortex -filetype=obj --arm-add-build-attributes -o arm64e.o arm64e.ll --mc-relax-all -mtriple=arm64e-apple-ios
~/llvm-apple/bin/llc -mcpu=cyclone -filetype=obj --arm-add-build-attributes -o arm64.o arm64.ll --mc-relax-all -mtriple=arm64-apple-ios
#~/llvm-apple/bin/llc -filetype=obj --arm-add-build-attributes -o armv7.o armv7.ll --mc-relax-all -mtriple=armv7-apple-ios
#~/llvm-apple/bin/llc -filetype=obj --arm-add-build-attributes -o armv7s.o armv7s.ll --mc-relax-all -mtriple=armv7s-apple-ios

#clang -c -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS13.5.sdk -Wl,-dead_strip -fobjc-arc -fmodules -framework Foundation -target arm64e-apple-ios -o wrapper-arm64e.o wrapper/wrapper.m
#clang -c -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS13.5.sdk -Wl,-dead_strip -fobjc-arc -fmodules -framework Foundation -target arm64-apple-ios -o wrapper-arm64.o wrapper/wrapper.m

#~/llvm-apple/bin/llvm-ar rc wrapper-arm64.a wrapper-arm64.o
#~/llvm-apple/bin/llvm-ar rc wrapper-arm64e.a wrapper-arm64e.o

clang -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS13.5.sdk -Wl,-dead_strip -nodefaultlibs -lc -lobjc -lresolv -framework Foundation -target arm64e-apple-ios -dynamiclib -undefined dynamic_lookup -o arm64e.dylib arm64e.o librustsupport.dylib
clang -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS13.5.sdk -Wl,-dead_strip -nodefaultlibs -lc -lobjc -lresolv -framework Foundation -target arm64-apple-ios -dynamiclib -undefined dynamic_lookup -o arm64.dylib arm64.o librustsupport.dylib
#clang -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS13.5.sdk -framework Foundation -target armv7-apple-ios -dynamiclib -undefined dynamic_lookup -o armv7.dylib armv7.o
#clang -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS13.5.sdk -framework Foundation -target armv7s-apple-ios -dynamiclib -undefined dynamic_lookup -o armv7s.dylib armv7s.o

lipo -create arm64e.dylib arm64.dylib -output ReachCCRust.dylib

ldid -S ReachCCRust.dylib

rm /Volumes/VMware\ Shared\ Folders/Shared/ReachCCRust.dylib
cp ReachCCRust.dylib /Volumes/VMware\ Shared\ Folders/Shared/ReachCCRust.dylib