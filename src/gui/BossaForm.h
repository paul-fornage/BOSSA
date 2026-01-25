///////////////////////////////////////////////////////////////////////////
// C++ code generated with wxFormBuilder (version 3.10.1-d4a3afd)
// http://www.wxformbuilder.org/
//
// PLEASE DO *NOT* EDIT THIS FILE!
///////////////////////////////////////////////////////////////////////////

#pragma once

#include <wx/artprov.h>
#include <wx/xrc/xmlres.h>
#include <wx/bitmap.h>
#include <wx/image.h>
#include <wx/icon.h>
#include <wx/statbmp.h>
#include <wx/gdicmn.h>
#include <wx/font.h>
#include <wx/colour.h>
#include <wx/settings.h>
#include <wx/string.h>
#include <wx/stattext.h>
#include <wx/button.h>
#include <wx/sizer.h>
#include <wx/combobox.h>
#include <wx/statbox.h>
#include <wx/filepicker.h>
#include <wx/statusbr.h>
#include <wx/frame.h>
#include <wx/gauge.h>
#include <wx/dialog.h>
#include <wx/hyperlink.h>
#include <wx/statline.h>
#include <wx/textctrl.h>
#include <wx/checkbox.h>

///////////////////////////////////////////////////////////////////////////


///////////////////////////////////////////////////////////////////////////////
/// Class MainFrame
///////////////////////////////////////////////////////////////////////////////
class MainFrame : public wxFrame
{
	private:

	protected:
		wxStaticBitmap* _bossaBitmap;
		wxStaticText* _titleText;
		wxButton* _aboutButton;
		wxComboBox* _portComboBox;
		wxButton* _refreshButton;
		wxFilePickerCtrl* _filePicker;
		wxButton* _writeButton;
		wxButton* _verifyButton;
		wxButton* _readButton;
		wxButton* _infoButton;
		wxButton* _exitButton;
		wxStatusBar* _statusBar;

	public:

		MainFrame( wxWindow* parent, wxWindowID id = wxID_ANY, const wxString& title = wxT("BOSSA - Kiibohd"), const wxPoint& pos = wxDefaultPosition, const wxSize& size = wxSize( 550,400 ), long style = wxCAPTION|wxCLOSE_BOX|wxICONIZE|wxMINIMIZE|wxMINIMIZE_BOX|wxSYSTEM_MENU|wxTAB_TRAVERSAL );

		~MainFrame();

};

///////////////////////////////////////////////////////////////////////////////
/// Class ProgressDialog
///////////////////////////////////////////////////////////////////////////////
class ProgressDialog : public wxDialog
{
	private:

	protected:
		wxStaticText* _infoStaticText;
		wxGauge* _statusGauge;
		wxStdDialogButtonSizer* _sdbSizer;
		wxButton* _sdbSizerCancel;

	public:

		ProgressDialog( wxWindow* parent, wxWindowID id = wxID_ANY, const wxString& title = wxT("Progress"), const wxPoint& pos = wxDefaultPosition, const wxSize& size = wxSize( 300,150 ), long style = wxCAPTION );

		~ProgressDialog();

};

///////////////////////////////////////////////////////////////////////////////
/// Class AboutDialog
///////////////////////////////////////////////////////////////////////////////
class AboutDialog : public wxDialog
{
	private:

	protected:
		wxStaticBitmap* _bossaBitmap;
		wxStaticText* _titleStaticText;
		wxHyperlinkCtrl* _githubHyperlink;
		wxStaticText* _versionStaticText;
		wxStaticText* _wxStaticText;
		wxStaticLine* m_staticline1;
		wxStaticBitmap* _shumatechBitmap;
		wxStaticText* _copyrightStaticText;
		wxHyperlinkCtrl* _shumatechHyperlink;
		wxStaticLine* m_staticline2;
		wxStaticText* m_disclaimerStaticText;
		wxStaticLine* m_staticline3;
		wxStdDialogButtonSizer* _sdbSizer;
		wxButton* _sdbSizerOK;

	public:

		AboutDialog( wxWindow* parent, wxWindowID id = wxID_ANY, const wxString& title = wxT("About BOSSA"), const wxPoint& pos = wxDefaultPosition, const wxSize& size = wxSize( 300,-1 ), long style = wxDEFAULT_DIALOG_STYLE );

		~AboutDialog();

};

///////////////////////////////////////////////////////////////////////////////
/// Class InfoDialog
///////////////////////////////////////////////////////////////////////////////
class InfoDialog : public wxDialog
{
	private:

	protected:
		wxStaticText* _deviceStaticText;
		wxTextCtrl* _deviceTextCtrl;
		wxStaticText* _versionStaticText;
		wxTextCtrl* _versionTextCtrl;
		wxStaticText* _pagesStaticText;
		wxTextCtrl* _pagesTextCtrl;
		wxStaticText* _pageSizeStaticText;
		wxTextCtrl* _pageSizeTextCtrl;
		wxStaticText* _totalSizeStaticText;
		wxTextCtrl* _totalSizeTextCtrl;
		wxStaticText* _planesStaticText;
		wxTextCtrl* _planesTextCtrl;
		wxCheckBox* _bootCheckBox;
		wxCheckBox* _bodCheckBox;
		wxCheckBox* _securityCheckBox;
		wxCheckBox* _borCheckBox;
		wxStaticText* _lockStaticText;
		wxTextCtrl* _lockTextCtrl;
		wxStdDialogButtonSizer* _sdbSizer;
		wxButton* _sdbSizerOK;

	public:

		InfoDialog( wxWindow* parent, wxWindowID id = wxID_ANY, const wxString& title = wxT("Info"), const wxPoint& pos = wxDefaultPosition, const wxSize& size = wxDefaultSize, long style = wxDEFAULT_DIALOG_STYLE );

		~InfoDialog();

};

