package main

import (
	"encoding/json"
	"github.com/galdor/go-cmdline"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
	"time"
)

//ProgramStatus is a representation of the rust code sanity check
type ProgramStatus struct {
	ProgramName  string `json:"Program Name:"`
	CleanStatus  string `json:"cargo clean status:"`
	CheckStatus  string `json:"cargo check status:"`
	TestStatus   string `json:"cargo test status:"`
	ClippyStatus string `json:"cargo clippy status:"`
	FmtStatus    string `json:"cargo fmt status:"`
}

var wg = sync.WaitGroup{}

func main() {
	start := time.Now()

	cmdline := cmdline.New()
	cmdline.AddOption("s", "search", "search-directory", "the input file")
	cmdline.AddOption("o", "output", "check-report.txt", "the output file")
	cmdline.AddOption("l", "log-file", "log.txt", "the log file")
	cmdline.Parse(os.Args)

	outputFile, err := os.Create(cmdline.OptionValue("o"))
	if err != nil {
		return
	}
	defer outputFile.Close()

	logFile, err := os.Create(cmdline.OptionValue("l"))
	if err != nil {
		log.Fatalf("Error creating file: %v", err)
	}
	defer logFile.Close()
	log.SetOutput(logFile)

	searchDir := cmdline.OptionValue("s")
	fileList := getFileList(searchDir)

	for _, filePath := range fileList {
		//To identify a rust program folder we look for Cargo.toml file
		if strings.Contains(filePath, "Cargo.toml") {
			wg.Add(1)
			programPath := strings.TrimSuffix(filePath, "Cargo.toml")
			go checkSanity(programPath, outputFile)
		}
	}
	wg.Wait()
	elapsed := time.Since(start)
	log.Printf("Total time taken to generate sanity check report: %s", elapsed)
}

func getFileList(searchDir string) []string {
	log.Println("Search Directory:", searchDir)

	fileList := []string{}
	//This will return the complete path of all the files present in the search directory
	//as an array of strings
	error := filepath.Walk(searchDir, func(path string, f os.FileInfo, err error) error {
		fileList = append(fileList, path)
		return nil
	})

	if error != nil {
		log.Printf("%s", error)
		os.Exit(1)
	}
	return fileList
}

func checkSanity(prgmPath string, outputFile *os.File) {
	cmdName := "cargo"
	os.Chdir(prgmPath)

	cleanArgs := []string{"clean"}
	clippyArgs := []string{"clippy"}
	checkArgs := []string{"check"}
	testArgs := []string{"test"}
	fmtArgs := []string{"fmt"}

	prgmReport := generateRpt(
		getProgramName(prgmPath),
		executeCommand(cmdName, cleanArgs...),
		executeCommand(cmdName, checkArgs...),
		executeCommand(cmdName, testArgs...),
		executeCommand(cmdName, clippyArgs...),
		executeCommand(cmdName, fmtArgs...))

	outputFile.WriteString(prgmReport)
	wg.Done()
}

func getProgramName(programPath string) string {
	programDir := strings.Split(programPath, "/")
	programName := programDir[len(programDir)-2]

	return programName
}

func executeCommand(name string, args ...string) string {
	error := exec.Command(name, args...).Run()
	if error != nil {
		log.Printf("%s", error)
		return "Fail"
	}
	return "Success"
}

func generateRpt(
	programName string,
	cleanCmdStatus string,
	checkCmdStatus string,
	testCmdStatus string,
	clippyCmdStatus string,
	fmtCmdStatus string) string {
	statusRpt := &ProgramStatus{
		ProgramName:  programName,
		CleanStatus:  cleanCmdStatus,
		CheckStatus:  checkCmdStatus,
		TestStatus:   testCmdStatus,
		ClippyStatus: clippyCmdStatus,
		FmtStatus:    fmtCmdStatus}

	report, err := json.Marshal(statusRpt)
	if err != nil {
		log.Fatal(err)
	}

	return string(report) + "\n"
}
