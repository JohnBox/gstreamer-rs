// Generated by gir (https://github.com/gtk-rs/gir @ 1bef39f)
// from gir-files (https://github.com/gtk-rs/gir-files @ 7d95377)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 831b444)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((guint) GST_RTSP_ADDRESS_FLAG_EVEN_PORT);
    PRINT_CONSTANT((guint) GST_RTSP_ADDRESS_FLAG_IPV4);
    PRINT_CONSTANT((guint) GST_RTSP_ADDRESS_FLAG_IPV6);
    PRINT_CONSTANT((guint) GST_RTSP_ADDRESS_FLAG_MULTICAST);
    PRINT_CONSTANT((guint) GST_RTSP_ADDRESS_FLAG_NONE);
    PRINT_CONSTANT((guint) GST_RTSP_ADDRESS_FLAG_UNICAST);
    PRINT_CONSTANT(GST_RTSP_ADDRESS_POOL_ANY_IPV4);
    PRINT_CONSTANT(GST_RTSP_ADDRESS_POOL_ANY_IPV6);
    PRINT_CONSTANT((gint) GST_RTSP_ADDRESS_POOL_EINVAL);
    PRINT_CONSTANT((gint) GST_RTSP_ADDRESS_POOL_ELAST);
    PRINT_CONSTANT((gint) GST_RTSP_ADDRESS_POOL_ERANGE);
    PRINT_CONSTANT((gint) GST_RTSP_ADDRESS_POOL_ERESERVED);
    PRINT_CONSTANT((gint) GST_RTSP_ADDRESS_POOL_OK);
    PRINT_CONSTANT(GST_RTSP_AUTH_CHECK_CONNECT);
    PRINT_CONSTANT(GST_RTSP_AUTH_CHECK_MEDIA_FACTORY_ACCESS);
    PRINT_CONSTANT(GST_RTSP_AUTH_CHECK_MEDIA_FACTORY_CONSTRUCT);
    PRINT_CONSTANT(GST_RTSP_AUTH_CHECK_TRANSPORT_CLIENT_SETTINGS);
    PRINT_CONSTANT(GST_RTSP_AUTH_CHECK_URL);
    PRINT_CONSTANT((gint) GST_RTSP_FILTER_KEEP);
    PRINT_CONSTANT((gint) GST_RTSP_FILTER_REF);
    PRINT_CONSTANT((gint) GST_RTSP_FILTER_REMOVE);
    PRINT_CONSTANT((gint) GST_RTSP_MEDIA_STATUS_ERROR);
    PRINT_CONSTANT((gint) GST_RTSP_MEDIA_STATUS_PREPARED);
    PRINT_CONSTANT((gint) GST_RTSP_MEDIA_STATUS_PREPARING);
    PRINT_CONSTANT((gint) GST_RTSP_MEDIA_STATUS_SUSPENDED);
    PRINT_CONSTANT((gint) GST_RTSP_MEDIA_STATUS_UNPREPARED);
    PRINT_CONSTANT((gint) GST_RTSP_MEDIA_STATUS_UNPREPARING);
    PRINT_CONSTANT(GST_RTSP_ONVIF_BACKCHANNEL_REQUIREMENT);
    PRINT_CONSTANT(GST_RTSP_ONVIF_REPLAY_REQUIREMENT);
    PRINT_CONSTANT(GST_RTSP_PERM_MEDIA_FACTORY_ACCESS);
    PRINT_CONSTANT(GST_RTSP_PERM_MEDIA_FACTORY_CONSTRUCT);
    PRINT_CONSTANT((gint) GST_RTSP_PUBLISH_CLOCK_MODE_CLOCK);
    PRINT_CONSTANT((gint) GST_RTSP_PUBLISH_CLOCK_MODE_CLOCK_AND_OFFSET);
    PRINT_CONSTANT((gint) GST_RTSP_PUBLISH_CLOCK_MODE_NONE);
    PRINT_CONSTANT((gint) GST_RTSP_SUSPEND_MODE_NONE);
    PRINT_CONSTANT((gint) GST_RTSP_SUSPEND_MODE_PAUSE);
    PRINT_CONSTANT((gint) GST_RTSP_SUSPEND_MODE_RESET);
    PRINT_CONSTANT((gint) GST_RTSP_THREAD_TYPE_CLIENT);
    PRINT_CONSTANT((gint) GST_RTSP_THREAD_TYPE_MEDIA);
    PRINT_CONSTANT(GST_RTSP_TOKEN_MEDIA_FACTORY_ROLE);
    PRINT_CONSTANT(GST_RTSP_TOKEN_TRANSPORT_CLIENT_SETTINGS);
    PRINT_CONSTANT((guint) GST_RTSP_TRANSPORT_MODE_PLAY);
    PRINT_CONSTANT((guint) GST_RTSP_TRANSPORT_MODE_RECORD);
    return 0;
}
