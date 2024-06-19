#include "../inc/files.hpp"

#include <iostream>

std::map<std::string, FILE*> File_Controller::shared_files =  std::map<std::string, FILE*>();

uint64_t File_Controller::next_file_id = 0;
std::map<uint64_t, std::string> File_Controller::shared_files_vm_ids =  std::map<uint64_t, std::string>();

std::map<uint64_t, std::map<std::string, uint64_t>> File_Controller::shared_files_cursors = std::map<uint64_t, std::map<std::string, uint64_t>>();

File_Controller::File_Controller() {}

File_Controller::~File_Controller() {}

void File_Controller::open_shared_files(std::vector<std::string> file_names) {

    for(auto &file : file_names) {

        FILE* fptr = fopen(file.c_str(), "r");

        File_Controller::shared_files[file] = fptr;

        File_Controller::shared_files_vm_ids[next_file_id++] = file;

    }

}

void File_Controller::close_shared_files(std::vector<std::string> file_names) {

    for(auto& file_pair : File_Controller::shared_files) {

        auto fptr = file_pair.second;

        fclose(fptr);

    }

}

void File_Controller::setup_shared_cursors(uint64_t vm_id) {

    std::map<std::string, uint64_t> t_map;

    for(auto file : File_Controller::shared_files) {
        std::string file_name = file.first;

        t_map[file_name] = 0;
    }


    File_Controller::shared_files_cursors[vm_id] = t_map;

}

bool File_Controller::gather_msg(uint8_t new_char) {

    // Sequence should look something like this
    // op_code(1 byte)
    // #
    // file_name/file_id
    // #
    // rw permisions/blank/data
    // #
    // #

    // We can count up to 4 # so we know where the message ended

    msg_buffer += new_char;

    // std::cout << msg_buffer << std::endl;

    uint64_t cnt = 0;
    for(auto c : msg_buffer) {
        if(c == '#') cnt+=1;
    }

    if(cnt == 4) {
        return false;
    }

    return true;
}

uint64_t File_Controller::get_operation() {

    // Sequence should look something like this
    // op_code(1 byte)
    // #
    // file_name/file_id
    // #
    // rw permisions/blank/data
    // #
    // #

    // Msg shoudl always be 1B#something#something##
    uint64_t op_code = static_cast<uint64_t>(msg_buffer[0]);

    return op_code;
}

// Function to convert std::string to uint64_t
uint64_t File_Controller::string_to_uint64(const std::string& str) {
    uint64_t result = 0;
    for (auto const& num : str)
    {
        result <<= 8;
        result |= num;
    }
    return result;

    return result;
}

// Function to convert uint64_t to std::string
std::string File_Controller::uint64_to_string(uint64_t value) {
    std::ostringstream oss;
    oss << value;
    return oss.str();
}

std::string File_Controller::invert_string(std::string to_invert) {
    std::string ret;
    for(int64_t i = to_invert.size() - 2; i>=0; i--) {
        ret += to_invert[i];
    }
    return ret;
}

uint64_t File_Controller::file_open(uint64_t vm_id) {

    // Sequence should look something like this
    // op_code(1 byte)
    // #
    // file_name
    // #
    // #
    // #

    size_t first_hash = msg_buffer.find('#');
    size_t second_hash = msg_buffer.find('#', first_hash + 1);
    size_t third_hash = msg_buffer.find('#', second_hash + 1);

     // Check if the delimiters are found
    if (first_hash != std::string::npos && second_hash != std::string::npos && third_hash != std::string::npos) {

        // Extract the substring between the second and third hashes
        std::string _file_name = msg_buffer.substr(first_hash + 1, second_hash - first_hash - 1);
        // Extract the substring between the second and third hashes
        std::string _permisions = msg_buffer.substr(second_hash + 1, third_hash - second_hash - 1);

        msg_buffer = "";

        std::string file_name = _file_name + uint64_to_string(vm_id);
        if(check_if_local(_file_name)) {
            // If it already exists return it

            uint64_t id = 0;
            for(auto e : local_files_vm_ids) {
                if(e.second == file_name) {
                    id = e.first;
                }
            }

            return id;
        
        }

        if(check_if_shared(_file_name)) {
            // If it only exists globaly return it

            uint64_t id = 0;
            for(auto e : shared_files_vm_ids) {
                if(e.second == _file_name) {
                    id = e.first;
                    break;
                }
            }

            return id;

        }

        // If it doenst exist create it

        _file_name += uint64_to_string(vm_id);

        FILE* ptr = fopen(_file_name.c_str(), "w+");

        local_files[_file_name] = ptr;
        
        uint64_t file_id = next_file_id;
        local_files_vm_ids[next_file_id++] = _file_name; 

        return file_id;

    } else {
        std::cerr << "Invalid format!" << std::endl;
    }

    msg_buffer = "";

    return 0;

}

std::string File_Controller::file_read(uint64_t vm_id) {

    // Sequence should look something like this
    // op_code(1 byte)
    // #
    // file_id
    // #
    // data_size
    // #
    // #

    size_t first_hash = msg_buffer.find('#');
    size_t second_hash = msg_buffer.find('#', first_hash + 1);
    size_t third_hash = msg_buffer.find('#', second_hash + 1);

    // Check if the delimiters are found
    if (first_hash != std::string::npos && second_hash != std::string::npos && third_hash != std::string::npos) {

        // Extract the substring between the second and third hashes
        std::string _file_id = msg_buffer.substr(first_hash + 1, second_hash - first_hash - 1);
        // Extract the substring between the second and third hashes
        std::string _data_size = msg_buffer.substr(second_hash + 1, third_hash - second_hash - 1);

        msg_buffer = "";


        uint64_t file_id = string_to_uint64(invert_string(_file_id));
        uint64_t data_size = string_to_uint64(invert_string(_data_size));

        char buffer[data_size];

        FILE* ptr = NULL;

        if(check_if_local(file_id)) {

            fseek(get_local_id(file_id), 0, SEEK_SET);
            uint64_t _size = fread(&buffer, sizeof(char), data_size , get_local_id(file_id));

            std::string send_it;
            for(int i = 0; i < _size; i++) {
                send_it += buffer[i];
            }

            return send_it;
        }

        if(check_if_shared(file_id)) {

            auto& cursors = File_Controller::shared_files_cursors[vm_id];

            fseek(get_shared_id(file_id), cursors[File_Controller::shared_files_vm_ids[file_id]], SEEK_SET);

            uint64_t _size = fread(&buffer, sizeof(char), data_size , get_shared_id(file_id));

            cursors[File_Controller::shared_files_vm_ids[file_id]] += _size;

            std::string send_it;
            for(int i = 0; i < _size; i++) {
                send_it += buffer[i];
            }
            

            return send_it;

        }

        std::cerr << "NO FILE FOUND FOR READ";

    } else {
        std::cerr << "Invalid format!" << std::endl;
    }

    msg_buffer = "";

    return "";

}

void File_Controller::file_write(uint64_t vm_id) {

    // Sequence should look something like this
    // op_code(1 byte)
    // #
    // file_id
    // #
    // data
    // #
    // #

    size_t first_hash = msg_buffer.find('#');
    size_t second_hash = msg_buffer.find('#', first_hash + 1);
    size_t third_hash = msg_buffer.find('#', second_hash + 1);

     // Check if the delimiters are found
    if (first_hash != std::string::npos && second_hash != std::string::npos && third_hash != std::string::npos) {

        // Extract the substring between the second and third hashes
        std::string _file_id = msg_buffer.substr(first_hash + 1, second_hash - first_hash - 1);
        // Extract the substring between the second and third hashes
        std::string data = msg_buffer.substr(second_hash + 1, third_hash - second_hash - 1);

        msg_buffer = "";

        uint64_t file_id = string_to_uint64(invert_string(_file_id));

        if(check_if_local(file_id)) {

            // If there si already a copy just write to it 

            fprintf(get_local_id(file_id), "%s", data.c_str());

            return;

        }

        if(check_if_shared(file_id)) {

            FILE* old_file = get_shared_id(file_id);

            std::string _file_name;

            for(auto e : shared_files) {
                if(e.second == old_file) {
                    _file_name = e.first;
                    break;
                }
            }

            std::string file_name = _file_name + uint64_to_string(vm_id);
            
            FILE* new_file = fopen(file_name.c_str(), "w+");

            auto& cursors = File_Controller::shared_files_cursors[vm_id];
            uint64_t cursor = cursors[file_name];

            int ch;
            fseek(get_shared_id(file_id), 0, SEEK_SET);
            while ((ch = fgetc(old_file)) != EOF) {
                fputc(ch, new_file);
            }
            fseek(get_shared_id(file_id), cursor, SEEK_SET);


            local_files_vm_ids[file_id] = file_name;
            local_files[file_name] = new_file;

            fseek(get_local_id(file_id), cursor, SEEK_SET);
            fprintf(get_local_id(file_id), "%s", data.c_str());

            return ;
        }

        std::cerr << "ERROR FILE DOESNT EXIST FOR WRITING";

    } else {
        std::cerr << "Invalid format!" << std::endl;
    }

    msg_buffer = "";
}

void File_Controller::file_close(uint64_t vm_id) {

    // Sequence should look something like this
    // op_code(1 byte)
    // #
    // file_id
    // #
    // #
    // #

    size_t first_hash = msg_buffer.find('#');
    size_t second_hash = msg_buffer.find('#', first_hash + 1);
    size_t third_hash = msg_buffer.find('#', second_hash + 1);

     // Check if the delimiters are found
    if (first_hash != std::string::npos && second_hash != std::string::npos && third_hash != std::string::npos) {
        // Extract the substring between the second and third hashes
        std::string file_id = msg_buffer.substr(first_hash + 1, second_hash - first_hash - 1);

        msg_buffer = "";

        uint64_t id = string_to_uint64(invert_string(file_id));

        if(check_if_local(id)) {
            fclose(get_local_id(id));
            return;
        }

        
        if(check_if_shared(id)) {
            return;
        }

        std::cerr << "Wrong file close";

    } else {
        std::cerr << "Invalid format!" << std::endl;
    }

    msg_buffer = "";
}

bool File_Controller::check_if_shared(uint64_t file_id) {

    auto it = File_Controller::shared_files_vm_ids.find(file_id);
    if(it != File_Controller::shared_files_vm_ids.end()) {
        return true;
    }

    return false;
}

bool File_Controller::check_if_shared(std::string file_name) {

    auto it = File_Controller::shared_files.find(file_name);

    if(it != File_Controller::shared_files.end()) {
        return true;
    }

    return false;
}

FILE* File_Controller::get_shared_id(uint64_t file_id) {
    return File_Controller::shared_files[File_Controller::shared_files_vm_ids[file_id]];
}

bool File_Controller::check_if_local(uint64_t file_id) {
    auto it = File_Controller::local_files_vm_ids.find(file_id);

    if(it != File_Controller::local_files_vm_ids.end()) {
        return true;
    }

    return false;
}

bool File_Controller::check_if_local(std::string file_name) {
    auto it = File_Controller::local_files.find(file_name);
    if(it != File_Controller::local_files.end()) {
        return true;
    }

    return false;
}

FILE* File_Controller::get_local_id(uint64_t file_id) {

    return local_files[local_files_vm_ids[file_id]];
}

