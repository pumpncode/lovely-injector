#include <stdio.h>
#include <stdint.h>

#include "dobby/include/dobby.h"

typedef enum
{
	kMemoryOperationSuccess,
	kMemoryOperationError,
	kNotSupportAllocateExecutableMemory,
	kNotEnough,
	kNone
} MemoryOperationError;

MemoryOperationError CodePatch(void *address, unsigned char *buffer, unsigned int buffer_size)
{
	int ret = DobbyCodePatch(address, buffer, buffer_size);
	if (ret == 0)
	{
		return kMemoryOperationSuccess;
	}
	else
	{
		return kMemoryOperationError;
	}
}
