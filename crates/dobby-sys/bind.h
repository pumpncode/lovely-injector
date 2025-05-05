// return dobby build date
const char *DobbyBuildVersion();

// replace function
int DobbyHook(void *address, void *replace_call, void **origin_call);
// destory and restore hook
int DobbyDestroy(void *address);

// iterate symbol table and find symbol
void *DobbySymbolResolver(const char *image_name, const char *symbol_name);

// global offset table
int DobbyGlobalOffsetTableReplace(char *image_name, char *symbol_name,
								  void *fake_func, void **orig_func);

typedef enum
{
	kMemoryOperationSuccess,
	kMemoryOperationError,
	kNotSupportAllocateExecutableMemory,
	kNotEnough,
	kNone
} MemoryOperationError;

#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

	MemoryOperationError CodePatch(void *address, unsigned char *buffer,
								   unsigned int buffer_size);

	int DobbyCodePatch(void *address, uint8_t *buffer, uint32_t buffer_size);

	void log_set_level(int level);
	void log_switch_to_syslog();
	void log_switch_to_file(const char *path);

#ifdef __cplusplus
}
#endif