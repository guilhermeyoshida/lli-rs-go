//go:build !windows

package lli

//go:generate bash -c "cd lli && cargo build --release"

/*
#cgo linux LDFLAGS: ${SRCDIR}/lli/target/release/liblli.a -ldl
#cgo darwin LDFLAGS: ${SRCDIR}/lli/target/release/liblli.a -ldl -framework Security -framework CoreFoundation

#include <stdlib.h>
extern char *hello_to_my_name(const char*);
extern unsigned int get_qtd_tokens(const char*, const char*);
*/
import "C"
import "unsafe"

func HelloToMyName(name string) string {
	n := C.CString(name)
	result := C.hello_to_my_name(n)
	goString := C.GoString(result)
	C.free(unsafe.Pointer(n))
	C.free(unsafe.Pointer(result))
	return goString
}

func CountToken(modelName, txt string) int {
	t := C.CString(txt)
	m := C.CString(modelName)
	count := C.get_qtd_tokens(m, t)
	C.free(unsafe.Pointer(t))
	C.free(unsafe.Pointer(m))
	return int(count)
}
