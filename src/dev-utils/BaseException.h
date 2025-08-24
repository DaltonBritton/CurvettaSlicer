#pragma once
#include <boost/nowide/cstdio.hpp>
#include <boost/nowide/fstream.hpp>
#include "stackwalker.h"
#include <eh.h>

class CBaseException : public CStackWalker
{
public:
	explicit CBaseException(HANDLE hProcess = GetCurrentProcess(), WORD wPID = GetCurrentProcessId(), LPCTSTR lpSymbolPath = nullptr, PEXCEPTION_POINTERS pEp = nullptr);
	~CBaseException();
	void OutputString(LPCTSTR lpszFormat, ...) override;
    [[maybe_unused]] virtual void ShowLoadModules();

    [[maybe_unused]] void ShowCallstack();
    void ShowCallstack(HANDLE hThread = GetCurrentThread());
    [[maybe_unused]] void ShowCallstack(const CONTEXT* context = nullptr);

	void ShowCallstack(HANDLE hThread, const CONTEXT* context);
	virtual void ShowExceptionResoult(DWORD dwExceptionCode);
	virtual BOOL GetLogicalAddress(PVOID addr, PTSTR szModule, DWORD len, DWORD& section, DWORD& offset );
	virtual void ShowRegistorInformation(PCONTEXT pCtx);
	virtual void ShowExceptionInformation();
	static LONG WINAPI UnhandledExceptionFilter(PEXCEPTION_POINTERS pExceptionInfo);
	static LONG WINAPI UnhandledExceptionFilter2(PEXCEPTION_POINTERS pExceptionInfo);
	static void STF(unsigned int ui,  PEXCEPTION_POINTERS pEp);
	//BBS set crash log folder
	static void set_log_folder(std::string log_folder);
protected:
	PEXCEPTION_POINTERS m_pEp;
	boost::nowide::ofstream *output_file;
};

#define SET_DEFULTER_HANDLER() SetUnhandledExceptionFilter(CBaseException::UnhandledExceptionFilter)

#define SET_DEFAUL_EXCEPTION() _set_se_translator(CBaseException::STF)