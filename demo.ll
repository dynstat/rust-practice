; ModuleID = 'demo.dc3ed5978db21c9e-cgu.0'
source_filename = "demo.dc3ed5978db21c9e-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h711c153b3c24c427E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he9cdc59753bd0c21E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he9cdc59753bd0c21E" }>, align 8
@alloc_85b5ec2b6dd9d9b3c047fa3c0ea49204 = private unnamed_addr constant [4 x i8] c"\0A\00\00\00", align 4
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\04\00\00\00\00\00\00\00\04\00\00\00\00\00\00\00", ptr @"_ZN37_$LT$i32$u20$as$u20$demo..MyTrait$GT$7do_work17he9351de4a1244c50E" }>, align 8
@alloc_28677088aabd366d68223ae0630d6486 = private unnamed_addr constant [8 x i8] c"\00\00\00\00\00\00$@", align 8
@vtable.2 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN37_$LT$f64$u20$as$u20$demo..MyTrait$GT$7do_work17h9f6619c23d305c8eE" }>, align 8

; <f64 as demo::MyTrait>::do_work
; Function Attrs: minsize mustprogress nofree norecurse nosync nounwind optsize willreturn memory(argmem: read) uwtable
define internal noundef i32 @"_ZN37_$LT$f64$u20$as$u20$demo..MyTrait$GT$7do_work17h9f6619c23d305c8eE"(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #0 {
start:
  %_3 = load double, ptr %self, align 8, !noundef !3
  %_2 = tail call i32 @llvm.fptosi.sat.i32.f64(double %_3)
  %_0 = mul i32 %_2, 3
  ret i32 %_0
}

; <i32 as demo::MyTrait>::do_work
; Function Attrs: minsize mustprogress nofree norecurse nosync nounwind optsize willreturn memory(argmem: read) uwtable
define internal noundef range(i32 0, -1) i32 @"_ZN37_$LT$i32$u20$as$u20$demo..MyTrait$GT$7do_work17he9351de4a1244c50E"(ptr noalias noundef readonly align 4 captures(none) dereferenceable(4) %self) unnamed_addr #0 {
start:
  %_2 = load i32, ptr %self, align 4, !noundef !3
  %_0 = shl i32 %_2, 1
  ret i32 %_0
}

; std::rt::lang_start
; Function Attrs: minsize optsize uwtable
define hidden noundef i64 @_ZN3std2rt10lang_start17hcda5c77054204648E(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #1 {
start:
  %_7 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7)
  store ptr %main, ptr %_7, align 8
; call std::rt::lang_start_internal
  %_0 = call noundef i64 @_ZN3std2rt19lang_start_internal17hf9be512cb4d4567aE(ptr noundef nonnull align 1 %_7, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7)
  ret i64 %_0
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint minsize optsize uwtable
define internal noundef i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he9cdc59753bd0c21E"(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #2 {
start:
  %_4 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys::backtrace::__rust_begin_short_backtrace
  tail call fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h70142d768e703d77E(ptr noundef nonnull %_4) #10
  ret i32 0
}

; std::sys::backtrace::__rust_begin_short_backtrace
; Function Attrs: minsize noinline optsize uwtable
define internal fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h70142d768e703d77E(ptr noundef nonnull readonly captures(none) %f) unnamed_addr #3 {
start:
  tail call void %f()
  tail call void asm sideeffect "", "~{memory}"() #11, !srcloc !4
  ret void
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint minsize optsize uwtable
define internal noundef i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h711c153b3c24c427E"(ptr noundef readonly captures(none) %_1) unnamed_addr #2 personality ptr @__CxxFrameHandler3 {
start:
  %0 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys::backtrace::__rust_begin_short_backtrace
  tail call fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h70142d768e703d77E(ptr noundef nonnull readonly %0) #10, !noalias !5
  ret i32 0
}

; demo::process_dynamic
; Function Attrs: minsize noinline optsize uwtable
define internal fastcc noundef i32 @_ZN4demo15process_dynamic17hc7706987329013a9E(ptr noundef nonnull align 1 %val.0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(32) %val.1) unnamed_addr #3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %val.1, i64 24
  %1 = load ptr, ptr %0, align 8, !invariant.load !3, !nonnull !3
  %_0 = tail call noundef i32 %1(ptr noundef nonnull align 1 %val.0) #12
  ret i32 %_0
}

; demo::process_generic
; Function Attrs: minsize mustprogress nofree noinline norecurse nosync nounwind optsize willreturn memory(none) uwtable
define internal fastcc noundef i32 @_ZN4demo15process_generic17h1c4798a783bb26edE() unnamed_addr #4 personality ptr @__CxxFrameHandler3 {
start:
  ret i32 30
}

; demo::process_generic
; Function Attrs: minsize mustprogress nofree noinline norecurse nosync nounwind optsize willreturn memory(none) uwtable
define internal fastcc noundef i32 @_ZN4demo15process_generic17h8dcd10b1d07c4b7fE() unnamed_addr #4 personality ptr @__CxxFrameHandler3 {
start:
  ret i32 20
}

; demo::main
; Function Attrs: minsize optsize uwtable
define hidden void @_ZN4demo4main17h193265a56c645843E() unnamed_addr #1 {
start:
  %_7 = alloca [16 x i8], align 4
; call demo::process_generic
  %a = tail call fastcc noundef i32 @_ZN4demo15process_generic17h8dcd10b1d07c4b7fE() #10
; call demo::process_generic
  %b = tail call fastcc noundef i32 @_ZN4demo15process_generic17h1c4798a783bb26edE() #10
; call demo::process_dynamic
  %c = tail call fastcc noundef i32 @_ZN4demo15process_dynamic17hc7706987329013a9E(ptr noundef nonnull align 1 @alloc_85b5ec2b6dd9d9b3c047fa3c0ea49204, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1) #10
; call demo::process_dynamic
  %d = tail call fastcc noundef i32 @_ZN4demo15process_dynamic17hc7706987329013a9E(ptr noundef nonnull align 1 @alloc_28677088aabd366d68223ae0630d6486, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.2) #10
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_7)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8)
  store i32 %a, ptr %_7, align 4, !alias.scope !11
  %_8.sroa.4.0._7.sroa_idx = getelementptr inbounds nuw i8, ptr %_7, i64 4
  store i32 %b, ptr %_8.sroa.4.0._7.sroa_idx, align 4, !alias.scope !11
  %_8.sroa.5.0._7.sroa_idx = getelementptr inbounds nuw i8, ptr %_7, i64 8
  store i32 %c, ptr %_8.sroa.5.0._7.sroa_idx, align 4, !alias.scope !11
  %_8.sroa.6.0._7.sroa_idx = getelementptr inbounds nuw i8, ptr %_7, i64 12
  store i32 %d, ptr %_8.sroa.6.0._7.sroa_idx, align 4, !alias.scope !11
  call void asm sideeffect "", "r,~{memory}"(ptr nonnull align 4 dereferenceable(16) %_7) #11, !noalias !8, !srcloc !4
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_7)
  ret void
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.fptosi.sat.i32.f64(double) #5

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #6

; std::rt::lang_start_internal
; Function Attrs: minsize optsize uwtable
declare noundef i64 @_ZN3std2rt19lang_start_internal17hf9be512cb4d4567aE(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #6

declare i32 @__CxxFrameHandler3(...) unnamed_addr #7

define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #8 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = tail call i64 @_ZN3std2rt10lang_start17hcda5c77054204648E(ptr @_ZN4demo4main17h193265a56c645843E, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #9

attributes #0 = { minsize mustprogress nofree norecurse nosync nounwind optsize willreturn memory(argmem: read) uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #1 = { minsize optsize uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #2 = { inlinehint minsize optsize uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #3 = { minsize noinline optsize uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #4 = { minsize mustprogress nofree noinline norecurse nosync nounwind optsize willreturn memory(none) uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #5 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #6 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #7 = { "target-cpu"="x86-64" }
attributes #8 = { "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #9 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #10 = { noinline }
attributes #11 = { nounwind }
attributes #12 = { inlinehint }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{!"rustc version 1.93.1 (01f6ddf75 2026-02-11)"}
!3 = !{}
!4 = !{i64 3645624076268426}
!5 = !{!6}
!6 = distinct !{!6, !7, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he9cdc59753bd0c21E: %_1"}
!7 = distinct !{!7, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he9cdc59753bd0c21E"}
!8 = !{!9}
!9 = distinct !{!9, !10, !"_ZN4core4hint9black_box17h52f7fc9636512599E: %dummy"}
!10 = distinct !{!10, !"_ZN4core4hint9black_box17h52f7fc9636512599E"}
!11 = !{!12, !9}
!12 = distinct !{!12, !10, !"_ZN4core4hint9black_box17h52f7fc9636512599E: %_0"}
