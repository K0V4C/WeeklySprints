#ifndef KVM_FILES_HPP
#define KVM_FILES_HPP

#include <vector>
#include <string>
#include <map>

#include <stdio.h>
#include <stdlib.h>

#include <sstream>

class File_Controller {
public:

    File_Controller();

    static uint64_t next_file_id;

    static std::map<std::string, FILE*> shared_files;
    static std::map<uint64_t, std::string> shared_files_vm_ids;

    static std::map<uint64_t, std::map<std::string, uint64_t>> shared_files_cursors; 

    // At the start of hypervisor we open and at end we close
    static void open_shared_files(std::vector<std::string> file_names);
    static void close_shared_files(std::vector<std::string> file_names);

    static void setup_shared_cursors(uint64_t vm_id);
    
    // Each supervisor thread has to first gather the msg
    bool gather_msg(uint8_t new_char);
    
    uint64_t static constexpr _fopen = 0x01;
    uint64_t static constexpr _fread = 0x02;
    uint64_t static constexpr _fwrite = 0x03;
    uint64_t static constexpr _fclose = 0x04;

    uint64_t get_operation();

    uint64_t string_to_uint64(const std::string& str);
    // Function to convert uint64_t to std::string
    std::string uint64_to_string(uint64_t value);

    std::string invert_string(std::string);

    uint64_t file_open(uint64_t vm_id);
    std::string file_read(uint64_t vm_id);
    void file_write(uint64_t vm_id);
    void file_close(uint64_t vm_id);

    // false file is not shared or doesnt exist
    bool check_if_shared(uint64_t file_id);
     bool check_if_shared(std::string file_name);
    FILE* get_shared_id(uint64_t file_id);

    // false file does not exist
    bool check_if_local(uint64_t file_id);
    bool check_if_local(std::string file_name);
    FILE* get_local_id(uint64_t file_id);


    // Flags for sending data to guest
    bool        num_bool = false;
    uint64_t    return_number = 0;
    uint64_t    num_cnt = 0;

    bool        data_bool = false;
    std::string return_data = "";
    uint64_t    data_cnt = 0;

    ~File_Controller();
private:

    std::map<std::string, FILE*> local_files; 
    std::map<uint64_t, std::string> local_files_vm_ids;


    std::string msg_buffer;

};

#endif